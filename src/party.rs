mod error;
mod role;

use std::cell::RefCell;
use std::rc::Rc;
pub use error::GMWError;
pub use role::Role;

use rand::{Rng, thread_rng};
use rand::distributions::Standard;

use crate::network::GMWConnection;
use crate::circuit::{Circuit, Gate, GateOperation};
use crate::mul_triple::MulTriple;
use crate::mul_triple::provider::MTProvider;
use crate::party::GMWError::ProtocolError;
use crate::network::GMWPacket;


pub struct Party<M: MTProvider, G: GMWConnection> {
    circuit: Circuit,
    role: Role,
    mtp: RefCell<M>,
    connection: Rc<RefCell<G>>,
}


fn generate_shares(input: &[bool]) -> (Vec<bool>, Vec<bool>) {
    let rng = thread_rng();

    let shared: Vec<bool> = rng.sample_iter(Standard).take(input.len()).collect();
    let own: Vec<bool> = input.iter().zip(shared.iter())
        .map(|(a, b)| a ^ b).collect();

    (own, shared)
}


impl<M: MTProvider, G: GMWConnection> Party<M, G> {
    /// Create a new party.
    pub fn new(
        circuit: Circuit, connection: Rc<RefCell<G>>, role: Role, mtp: M,
    ) -> Self {
        Party {
            circuit,
            connection,
            role,
            mtp: RefCell::new(mtp)
        }
    }

    fn compute_and(&self, x: bool, y: bool) -> Result<bool, GMWError> {
        let MulTriple { a, b, c } = self.mtp.borrow_mut().get_triple();

        let (d1, e1) = (x ^ a, y ^ b);

        let GMWPacket::And { d: d2, e: e2 } = self.connection.borrow_mut().exchange(
            GMWPacket::And { d: d1, e: e1 }
        )? else { return Err(ProtocolError); };

        let (d, e) = (d1 ^ d2, e1 ^ e2);

        if self.role == Role::Server {
            Ok(d & b ^ e & a ^ c ^ d & e)
        } else {
            Ok(d & b ^ e & a ^ c)
        }
    }

    /// Executes the GMW protocol with the linked party for the stored circuit.
    pub fn execute(&mut self, input: &[bool]) -> Result<Vec<bool>, GMWError> {
        if input.len() != self.circuit.header.wires_per_input[self.role.index()] {
            return Err(GMWError::InputLengthMismatch {
                actual: input.len(),
                expected: self.circuit.header.wires_per_input[self.role.index()],
            });
        }

        let (own_share, pub_share) = generate_shares(input);
        let GMWPacket::ParameterShares(partner_share) = self.connection
            .borrow_mut()
            .exchange(GMWPacket::ParameterShares(pub_share))? else {
            return Err(ProtocolError);
        };


        let mut wire_shares = vec![false; self.circuit.header.num_wires];

        let own_input_range = self.circuit.parameter_range(self.role.index());
        let partner_input_range = self.circuit.parameter_range((!self.role).index());
        wire_shares[own_input_range].copy_from_slice(&own_share);
        wire_shares[partner_input_range].copy_from_slice(&partner_share);

        for &Gate { op, output_wire } in &self.circuit.gates {
            match op {
                GateOperation::XOR(a, b) => {
                    let (a, b) = (wire_shares[a], wire_shares[b], );
                    wire_shares[output_wire] = a ^ b;
                }

                GateOperation::INV(x) => {
                    let x = wire_shares[x];
                    if self.role == Role::Server {
                        wire_shares[output_wire] = !x;
                    } else {
                        wire_shares[output_wire] = x;
                    }
                }

                GateOperation::AND(a, b) => {
                    let (a, b) = (wire_shares[a], wire_shares[b]);
                    wire_shares[output_wire] = self.compute_and(a, b)?;
                }

                g => { return Err(GMWError::InvalidGate(g)); }
            }
        }

        let output_offset = self.circuit.header.num_wires - self.circuit.output_bit_count();

        let d1: Vec<bool> = wire_shares[output_offset..].into();

        let GMWPacket::Result(d2) = self.connection
            .borrow_mut()
            .exchange(GMWPacket::Result(d1.clone()))? else { return Err(ProtocolError); };

        Ok(d1.iter().zip(d2).map(|(x, y)| x ^ y).collect())
    }
}