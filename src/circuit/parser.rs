use crate::circuit::{Circuit, Gate, GateOperation, Header};
use crate::circuit::error::ParserError;
use crate::circuit::error::ParserError::Syntax;

fn parse_n_numbers<'a>(words: &mut impl Iterator<Item=&'a str>, n: usize)
    -> Result<Vec<usize>, ParserError> {
    let mut numbers: Vec<usize> = Vec::with_capacity(n);

    for _ in 0..n {
        numbers.push(
            if let Some(n) = words.next(){
                n.parse::<usize>()?
            } else {
                return Err(ParserError::EndOfLine);
            }
        )
    }
    debug_assert!(numbers.len() == n);

    Ok(numbers)
}

fn parse_line(line: &str) -> Result<Vec<usize>, ParserError> {
    let mut v: Vec<usize> = Vec::new();
    for word in line.split(' ') {
        v.push(word.parse()?)
    }
    Ok(v)
}

fn get_next_line(iter: &mut impl Iterator<Item=String>) -> Result<String, ParserError> {
    if let Some(s) = iter.next() {
        Ok(s)
    } else {
        Err(ParserError::EndOfLine)
    }
}

pub fn parse_header(lines: &mut impl Iterator<Item=String>) -> Result<Header, ParserError> {
    let [num_gates, num_wires] = parse_n_numbers(&mut get_next_line(lines)?.split(' '), 2)?
        .try_into()
        .unwrap();

    let niv_line = parse_line(&get_next_line(lines)?)?;
    let Some(&niv) = niv_line.first() else {
        return Err(ParserError::EndOfLine);
    };
    if niv_line.len() - 1 != niv {
        return Err(ParserError::Syntax(
            format!("Size of input wire list does not match niv: niv={niv}, {}", niv_line.len())
        ));
    }
    let wires_per_input = niv_line[1..].to_vec();

    let nov_line = parse_line(&get_next_line(lines)?)?;
    let Some(&nov) = nov_line.first() else {
        return Err(ParserError::EndOfLine);
    };
    if nov_line.len() - 1 != nov {
        return Err(ParserError::Syntax(
            format!("Size of output wire list does not match nov: nov={nov}, {}", nov_line.len())
        ));
    }
    let wires_per_output = niv_line[1..].to_vec();

    Ok(Header { num_gates, num_wires, wires_per_input, wires_per_output })
}

pub fn parse_gate(gate_repr: &str) -> Result<Gate, ParserError> {
    let mut iter = gate_repr.split(' ');

    let [n_in_wires, n_out_wires] = parse_n_numbers(&mut iter, 2)?.try_into().unwrap();

    let in_wires: Vec<usize> = parse_n_numbers(&mut iter, n_in_wires)?;
    let out_wires: Vec<usize> = parse_n_numbers(&mut iter, n_out_wires)?;

    assert_eq!(n_out_wires, 1, "MAND Gates not implemented yet!"); // TODO MAND

    let Some(operation) = iter.next() else {
        return Err(ParserError::EndOfFile);
    };

    let gate = match operation {
        "XOR" => {
            if in_wires.len() != 2 {
                return Err(Syntax("XOR Gate requires two inputs".into()));
            }
            GateOperation::XOR(in_wires[0], in_wires[1])
        },
        "AND" => {
            if in_wires.len() != 2 {
                return Err(Syntax("AND Gate requires two inputs".into()));
            }
            GateOperation::AND(in_wires[0], in_wires[1])
        },
        "INV" => {
            if in_wires.len() != 1 {
                return Err(Syntax("INV Gate requires one input".into()));
            }
            GateOperation::INV(in_wires[0])
        },
        "EQ" => {
            if in_wires.len() != 1 {
                return Err(Syntax("EQ Gate requires one input".into()));
            }
            GateOperation::EQ { constant: in_wires[0] == 1 }
        }
        "EQW" => {
            if in_wires.len() != 1 { 
                return Err(Syntax("EQW Gate requires one input".into()));
            }
            GateOperation::EQW(in_wires[0]) 
        },

        // TODO: "MAND" => ...
        g => return Err(Syntax(format!("Unknown Gate type: {g}"))),
    };

    if out_wires.len() != 1 {
        return Err(Syntax("MAND Gates not implemented yet".into()))
    }

    Ok(Gate {
        op: gate,
        output_wire: out_wires[0],
    })
}

pub fn parse(circuit: &mut impl Iterator<Item=String>) -> Result<Circuit, ParserError> {
    let mut lines = circuit;

    let header = parse_header(&mut lines)?;

    // skip empty line
    assert_eq!(
        lines.next().expect("Expected empty line, got EOF"), "",
        "Expected empty line"
    );

    let mut gates = Vec::with_capacity(header.num_gates);

    for _ in 0..header.num_gates {
        let gate = parse_gate(&lines.next().expect("Expected gate, got EOF"))?;
        gates.push(gate);
    }

    Ok(Circuit { header, gates })
}
