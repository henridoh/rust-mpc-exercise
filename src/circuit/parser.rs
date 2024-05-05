use crate::circuit::{Circuit, Gate, GateOperation, Header};
use crate::circuit::error::ParserError::{Syntax as SyntaxErr, Token as TokenErr, self};
use crate::circuit::tokenizer::{self, TokenStream, LexicalUnit};


pub fn parse_header(token_stream: &mut TokenStream) -> Result<Header, ParserError> {
    let (num_gates, num_wires) = (token_stream.accept_number()?, token_stream.accept_number()?);
    token_stream.accept_newline()?;

    let niv = token_stream.accept_number()?;
    let wires_per_input = token_stream.accept_n_numbers(niv)?;
    token_stream.accept_newline()?;


    let nov = token_stream.accept_number()?;
    let wires_per_output = token_stream.accept_n_numbers(nov)?;
    token_stream.accept_newline()?;

    Ok(Header { num_gates, num_wires, wires_per_input, wires_per_output })
}

pub fn parse_gate(token_stream: &mut TokenStream) -> Result<Gate, ParserError> {
    let location = token_stream.current_location();

    let n_in_wires = token_stream.accept_number()?;
    let n_out_wires = token_stream.accept_number()?;


    let in_wires = token_stream.accept_n_numbers(n_in_wires)?;
    let out_wires = token_stream.accept_n_numbers(n_out_wires)?;

    let operation = token_stream.accept_identifier()?;

    let gate = match operation.as_str() {
        "XOR" => {
            if in_wires.len() != 2 {
                return Err(SyntaxErr { message: "XOR Gate requires two inputs".into(), location });
            }
            GateOperation::XOR(in_wires[0], in_wires[1])
        }
        "AND" => {
            if in_wires.len() != 2 {
                return Err(SyntaxErr { message: "AND Gate requires two inputs".into(), location });
            }
            GateOperation::AND(in_wires[0], in_wires[1])
        }
        "INV" => {
            if in_wires.len() != 1 {
                return Err(SyntaxErr { message: "INV Gate requires one input".into(), location });
            }
            GateOperation::INV(in_wires[0])
        }
        "EQ" => {
            if in_wires.len() != 1 {
                return Err(SyntaxErr { message: "EQ Gate requires one input".into(), location });
            }
            GateOperation::EQ { constant: in_wires[0] == 1 }
        }
        "EQW" => {
            if in_wires.len() != 1 {
                return Err(SyntaxErr { message: "EQW Gate requires one input".into(), location });
            }
            GateOperation::EQW(in_wires[0])
        }

        g => return Err(SyntaxErr { message: format!("Unknown Gate Type: {}", g), location }),
    };

    // skip newline or eof.
    match token_stream.accept_newline() {
        Err(TokenErr {
                actual: tokenizer::Token { value: LexicalUnit::NewLine, .. }, ..
            }) | Ok(_) => (),

        Err(e) => { return Err(e); }
    }

    if out_wires.len() != 1 {
        Err(SyntaxErr {
            message: format!("Gate of Type {} must have one output wire", operation),
            location,
        })
    } else {
        Ok(Gate {
            op: gate,
            output_wire: out_wires[0],
        })
    }
}

pub fn parse(circuit: &mut dyn Iterator<Item=char>) -> Result<Circuit, ParserError> {
    let mut token_stream = TokenStream::new(circuit);

    let header = parse_header(&mut token_stream)?;

    // skip empty line
    token_stream.accept_newline()?;

    let mut gates = Vec::with_capacity(header.num_gates);

    for _ in 0..header.num_gates {
        let gate = parse_gate(&mut token_stream)?;
        gates.push(gate);
    }

    Ok(Circuit { header, gates })
}
