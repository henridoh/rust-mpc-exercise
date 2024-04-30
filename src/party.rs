mod error;

pub use error::GMWError;

use std::cell::RefCell;
use std::sync::mpsc::{Sender, Receiver, channel};
use crate::circuit::{Circuit, Gate};
use rand::{Rng, thread_rng};
use rand::distributions::Standard;
use crate::mul_triple::{MTProvider, MulTriple, TrivialMTP};


#[derive(Debug)]
enum MessageType {
    InputShares {
        role: Role,
        shares: Vec<bool>,
    },
    And {
        d: bool,
        e: bool,
    },
    Result(Vec<bool>),
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Role {
    P0,
    P1,
}

impl Role {
    pub fn index(&self) -> usize {
        match self {
            Role::P0 => 0,
            Role::P1 => 1,
        }
    }
    pub fn parameter_offset(&self, circuit: &Circuit) -> usize {
        circuit.offset_of_parameter(self.index())
    }
}

pub struct Party<T: MTProvider> {
    circuit: Circuit,
    sender: Sender<MessageType>,
    receiver: Receiver<MessageType>,
    role: Role,
    mtp: RefCell<T>,
}


/// Creates a new pair of parties for the provided circuit that can communicate with each other
/// to execute the provided circuit.
pub fn new_party_pair(circuit: Circuit) -> (Party<TrivialMTP>, Party<TrivialMTP>) {
    let (a_send, b_recv) = channel();
    let (b_send, a_recv) = channel();

    let mtp_a = TrivialMTP {};
    let mtp_b = TrivialMTP {};

    (Party::new(circuit.clone(), a_send, a_recv, Role::P0, mtp_a),
     Party::new(circuit, b_send, b_recv, Role::P1, mtp_b))
}

fn generate_shares(input: &[bool]) -> (Vec<bool>, Vec<bool>) {
    let rng = thread_rng();

    let shared: Vec<bool> = rng.sample_iter(Standard).take(input.len()).collect();
    let own: Vec<bool> = input.iter().zip(shared.iter())
        .map(|(a, b)| a ^ b).collect();

    (own, shared)
}


impl<T: MTProvider> Party<T> {
    /// Create a new party.
    fn new(
        circuit: Circuit, send_on: Sender<MessageType>,
        recv_on: Receiver<MessageType>, role: Role, mtp: T,
    ) -> Self {
        Party { circuit, sender: send_on, receiver: recv_on, role, mtp: RefCell::new(mtp) }
    }

    fn compute_and(&self, x: bool, y: bool) -> Result<bool, GMWError> {
        let MulTriple { a, b, c } = self.mtp.borrow_mut().get_triple();

        let (d1, e1) = (x ^ a, y ^ b);

        self.sender.send(MessageType::And { d: d1, e: e1 })?;
        let MessageType::And { d: d2, e: e2 } = self.receiver.recv()? else {
            return Err(GMWError::ProtocolError);
        };

        let (d, e) = (d1 ^ d2, e1 ^ e2);

        if self.role == Role::P0 {
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

        let (priv_input_share, pub_input_share) = generate_shares(input);

        self.sender.send(
            MessageType::InputShares { role: self.role, shares: pub_input_share }
        )?;
        let MessageType::InputShares { 
            role: partner_role, shares: partner_share
        } = self.receiver.recv()? else { return Err(GMWError::ProtocolError); };
        
        let mut wire_shares = vec![None; circuit.header.num_wires];

        let own_offset = self.role.parameter_offset(circuit);
        for (i, &v) in priv_input_share.iter().enumerate() {
            wire_shares[own_offset + i] = Some(v);
        }

        let partner_offset = partner_role.parameter_offset(circuit);
        for (i, &v) in partner_share.iter().enumerate() {
            wire_shares[partner_offset + i] = Some(v);
        }


        for (gate, out_wire) in &circuit.gates {
            let out_wire: usize = *out_wire;
            match gate {
                &Gate::XOR(a, b) => {
                    let (a, b) = (
                        wire_shares[a].unwrap_or_else(|| panic!("Wire {} should be set by now", a)),
                        wire_shares[b].unwrap_or_else(|| panic!("Wire {} should be set by now", b)),
                    );
                    wire_shares[out_wire] = Some(a ^ b);
                }

                &Gate::INV(x) => {
                    let x = wire_shares[x]
                        .unwrap_or_else(|| panic!("Wire {} should be set by now", x));
                    if self.role == Role::P0 {
                        wire_shares[out_wire] = Some(!x);
                    } else {
                        wire_shares[out_wire] = Some(x);
                    }
                }

                &Gate::AND(a, b) => {
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

        self.sender.send(MessageType::Result(d1.clone()))?;
        let MessageType::Result(d2) = self.receiver.recv()? else {
            return Err(GMWError::ProtocolError);
        };

        Ok(d1.iter().zip(d2.iter()).map(|(x, y)| x ^ y).collect())
    }
}