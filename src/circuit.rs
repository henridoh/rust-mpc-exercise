use std::fmt::Debug;
use std::ops::Range;
use crate::circuit::error::ParserError;

mod parser;
mod error;
mod tokenizer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateOperation {
    XOR(usize, usize),
    AND(usize, usize),
    INV(usize),
    EQ { constant: bool },
    EQW(usize),
    // TODO MAND {...},
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        return Self::parse_stream(&mut circuit.chars());
    }

    pub fn parse_stream(circuit: &mut dyn Iterator<Item=char>) -> Result<Self, ParserError> {
        parser::parse(circuit)
    }

    pub fn input_bit_count(&self) -> usize {
        self.header.wires_per_input.iter().sum()
    }

    pub fn output_bit_count(&self) -> usize {
        self.header.wires_per_output.iter().sum()
    }

    pub fn parameter_offset(&self, parameter_index: usize) -> usize {
        self.header.wires_per_input.iter()
            .take(parameter_index)
            .sum()
    }
    
    pub fn parameter_range(&self, parameter_index: usize) -> Range<usize> {        
        let offset = self.parameter_offset(parameter_index);
        let size = self.header.wires_per_input[parameter_index];
        
        offset..offset+size
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
            2 4\n\
            2 1 1\n\
            1 1\n\
            \n\
            2 1 0 1 2 AND\n\
            2 1 1 2 3 XOR\n";

        let circuit = Circuit::parse(source).unwrap();

        assert_eq!(
            circuit.gates,
            vec![
                Gate{ op: GateOperation::AND(0, 1), output_wire: 2}, 
                Gate{ op: GateOperation::XOR(1, 2), output_wire: 3}
            ]
        );
    }
}