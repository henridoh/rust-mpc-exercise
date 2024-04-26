use std::sync::mpsc::{Sender, Receiver, channel};
use crate::circuit::Circuit;



struct Party {
    circuit: Circuit,

    sender: Sender<bool>,
    receiver: Receiver<bool>,
}


/// Creates a new pair of parties for the provided circuit that can communicate with each other
/// to execute the provided circuit.
pub fn new_party_pair(circuit: Circuit) -> (Party, Party) {
    let (a_send, b_recv) = channel();
    let (b_send, a_recv) = channel();

    let a = Party {
        circuit: circuit.clone(),
        sender: a_send,
        receiver: a_recv,
    };
    
    let b = Party {
        circuit,
        sender: b_send,
        receiver: b_recv,
    };

    (a, b)
}


pub enum Role {
    Client, Server,
}

impl Party {
    /// Create a new party.
    pub fn new(circuit: (), role: ) -> Self {
        todo!()
    }

    /// Executes the GMW protocol with the linked party for the stored circuit.
    pub fn execute(&mut self, input: Vec<bool>) -> Result<Vec<bool>, ()> { // TODO change error type
        // Iterate over the stored circuit in topological order. `match` on the gate type and
        // evaluate it, potentially using a multiplication triple for and And Gate and communication
        // over the shared channel.
        todo!()
    }
}