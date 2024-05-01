use std::fmt::Debug;
use crate::circuit::error::ParserError;

mod parser;
mod error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateOperation {
    XOR(usize, usize),
    AND(usize, usize),
    INV(usize),
    EQ { constant: bool },
    EQW(usize),
    // TODO MAND {...},
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gate {
    pub op: GateOperation,
    pub output_wire: usize,
}

#[derive(Debug, Clone)]
pub struct Header {
    pub num_gates: usize,
    pub num_wires: usize,

    pub wires_per_input: Vec<usize>,
    pub wires_per_output: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Circuit {
    pub header: Header,
    pub gates: Vec<Gate>,
}


impl Circuit {
    /// Parses the bristol file contents into a circuit
    pub fn parse(circuit: &str) -> Result<Self, ParserError> {
        return Self::parse_lines(&mut circuit.lines().map(|s| s.to_string()));
    }

    pub fn parse_lines(circuit: &mut impl Iterator<Item=String>) -> Result<Self, ParserError> {
        parser::parse(circuit)
    }

    pub fn input_bit_count(&self) -> usize {
        self.header.wires_per_input.iter().sum()
    }

    pub fn output_bit_count(&self) -> usize {
        self.header.wires_per_output.iter().sum()
    }

    pub fn offset_of_parameter(&self, parameter_index: usize) -> usize {
        self.header.wires_per_input.iter()
            .take(parameter_index)
            .sum()
    }

    pub fn and_count(&self) -> usize {
        self.gates.iter().filter(
            |w| matches!(w, Gate { op: GateOperation::AND(_, _), ..})
        ).count()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Functions marked with `#[test]` are automatically run when you execute `cargo test`.
    #[test]
    fn test_simple_and() {
        let source = "\
            1 3\n\
            2 1 1\n\
            1 1\n\
            \n\
            2 1 0 1 2 AND\n";

        let circuit = Circuit::parse(source).unwrap();

        assert_eq!(
            circuit.gates,
            vec![Gate{ op: GateOperation::AND(0, 1), output_wire: 2} ]
        );
    }
}