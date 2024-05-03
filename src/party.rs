mod error;
mod network;

pub use error::GMWError;

use network::GMWConnection;

use std::cell::RefCell;
use std::sync::mpsc::{channel};
use crate::circuit::{Circuit, Gate, GateOperation};
use rand::{Rng, RngCore, thread_rng};
use rand::distributions::Standard;
use rand::rngs::StdRng;
use crate::mul_triple::{MTProvider, MulTriple, SharedSeedMTP};
use crate::party::GMWError::ProtocolError;
use crate::party::network::{MemChannelConnection, MessageType};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Role {
    Server,
    Client,
}

impl Role {
    pub fn index(&self) -> usize {
        match self {
            Role::Server => 0,
            Role::Client => 1,
        }
    }
    pub fn parameter_offset(&self, circuit: &Circuit) -> usize {
        circuit.offset_of_parameter(self.index())
    }
}

pub struct Party<M: MTProvider, C: GMWConnection> {
    circuit: Circuit,
    role: Role,
    mtp: RefCell<M>,
    connection: C,
}

/// Creates a new pair of parties for the provided circuit that can communicate with each other
/// to execute the provided circuit.
pub fn new_party_pair(circuit: Circuit)
                      -> (Party<SharedSeedMTP<StdRng>, MemChannelConnection>,
                          Party<SharedSeedMTP<StdRng>, MemChannelConnection>) {
    let (a_send, b_recv) = channel();
    let (b_send, a_recv) = channel();

    let a_connection = MemChannelConnection {
        sender: a_send,
        receiver: a_recv,
    };

    let b_connection = MemChannelConnection {
        sender: b_send,
        receiver: b_recv,
    };

    let mut seed: [u8; 32] = Default::default();
    thread_rng().fill_bytes(&mut seed);

    let mtp_a: SharedSeedMTP<StdRng> = SharedSeedMTP::new(seed);
    let mtp_b: SharedSeedMTP<StdRng> = SharedSeedMTP::new(seed);

    (Party::new(circuit.clone(), a_connection, Role::Server, mtp_a),
     Party::new(circuit, b_connection, Role::Client, mtp_b))
}

fn generate_shares(input: &[bool]) -> (Vec<bool>, Vec<bool>) {
    let rng = thread_rng();

    let shared: Vec<bool> = rng.sample_iter(Standard).take(input.len()).collect();
    let own: Vec<bool> = input.iter().zip(shared.iter())
        .map(|(a, b)| a ^ b).collect();

    (own, shared)
}


impl<M: MTProvider, C: GMWConnection> Party<M, C> {
    /// Create a new party.
    fn new(
        circuit: Circuit, connection: C, role: Role, mtp: M,
    ) -> Self { Party { circuit, connection, role, mtp: RefCell::new(mtp) } }

    fn compute_and(&self, x: bool, y: bool) -> Result<bool, GMWError> {
        let MulTriple { a, b, c } = self.mtp.borrow_mut().get_triple();

        let (d1, e1) = (x ^ a, y ^ b);

        let MessageType::And { d: d2, e: e2 } = self.connection.exchange(
            MessageType::And { d: d1, e: e1 }
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
        let circuit = &self.circuit;

        if input.len() != circuit.header.wires_per_input[self.role.index()] {
            return Err(GMWError::InputLengthMismatch {
                actual: input.len(),
                expected: circuit.header.wires_per_input[self.role.index()],
            });
        }


        let (own_share, pub_share) = generate_shares(input);
        let MessageType::ParameterShares(partner_share) = self.connection.exchange(
            MessageType::ParameterShares(pub_share)
        )? else {
            return Err(ProtocolError)
        };
        let partner_role = if self.role == Role::Server { Role::Client } else { Role::Server };

        let mut wire_shares = vec![None; circuit.header.num_wires];

        let own_offset = self.role.parameter_offset(circuit);
        for (i, &v) in own_share.iter().enumerate() {
            wire_shares[own_offset + i] = Some(v);
        }

        let partner_offset = partner_role.parameter_offset(circuit);
        for (i, &v) in partner_share.iter().enumerate() {
            wire_shares[partner_offset + i] = Some(v);
        }


        for Gate { op: gate, output_wire } in &circuit.gates {
            let out_wire: usize = *output_wire;
            match gate {
                &GateOperation::XOR(a, b) => {
                    let (a, b) = (
                        wire_shares[a].unwrap_or_else(|| panic!("Wire {} should be set by now", a)),
                        wire_shares[b].unwrap_or_else(|| panic!("Wire {} should be set by now", b)),
                    );
                    wire_shares[out_wire] = Some(a ^ b);
                }

                &GateOperation::INV(x) => {
                    let x = wire_shares[x]
                        .unwrap_or_else(|| panic!("Wire {} should be set by now", x));
                    if self.role == Role::Server {
                        wire_shares[out_wire] = Some(!x);
                    } else {
                        wire_shares[out_wire] = Some(x);
                    }
                }

                &GateOperation::AND(a, b) => {
                    let (a, b) = (
                        wire_shares[a].unwrap_or_else(|| panic!("Wire {} should be set by now", a)),
                        wire_shares[b].unwrap_or_else(|| panic!("Wire {} should be set by now", b)),
                    );
                    wire_shares[out_wire] = Some(self.compute_and(a, b)?);
                }

                g => { return Err(GMWError::InvalidGate(g.clone())); }
            }
        }


        let output_offset = circuit.header.num_wires - circuit.output_bit_count();

        let d1: Vec<bool> = wire_shares.into_iter()
            .skip(output_offset)
            .map(Option::unwrap)
            .collect();

        let MessageType::Result(d2) = self.connection.exchange(
            MessageType::Result(d1.clone())
        )? else { return Err(ProtocolError); };

        Ok(d1.iter().zip(d2.iter()).map(|(x, y)| x ^ y).collect())
    }
}