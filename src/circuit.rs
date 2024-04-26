#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Gate {
    XOR(u32, u32),
    AND(u32, u32),
    INV(u32),
    EQ { constant: bool },
    EQW(u32),
    // TODO MAND {...},
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Wire {
    Input,
    Gate(Gate),
}


#[derive(Debug, Clone)]
pub struct Header {
    num_gates: u32,
    num_wires: u32,

    wires_per_input: Vec<u32>,
    wires_per_output: Vec<u32>,
}

#[derive(Clone)]
pub struct Circuit {
    wires: Vec<Wire>,
}

fn parse_header<'a>(lines: &mut impl Iterator<Item=&'a str>) -> Header {
    let mut line_iter = lines.next().unwrap().split(' ');
    let num_gates: u32 = line_iter.next().unwrap().parse().unwrap();
    let num_wires: u32 = line_iter.next().unwrap().parse().unwrap();
    assert!(line_iter.next().is_none());

    let mut line_iter = lines.next().unwrap().split(' ');
    let niv: u32 = line_iter.next().unwrap().parse().unwrap();
    let wires_per_input: Vec<u32> = line_iter
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(wires_per_input.len(), niv as usize);

    let mut line_iter = lines.next().unwrap().split(' ');
    let nov: u32 = line_iter.next().unwrap().parse().unwrap();
    let wires_per_output: Vec<u32> = line_iter
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(wires_per_output.len(), nov as usize);

    Header {
        num_gates, num_wires, wires_per_input, wires_per_output
    }
}

fn parse_gate(gate_repr: &str, wires: &mut Vec<Wire>) {
    let mut iter = gate_repr.split(' ');

    let n_in_wires: u32 = iter.next().unwrap().parse().unwrap();
    assert!(n_in_wires == 1 || n_in_wires == 2);
    let n_out_wires: u32 = iter.next().unwrap().parse().unwrap();

    let mut in_wires: Vec<u32> = Vec::with_capacity(n_in_wires as usize);
    let mut out_wires: Vec<u32> = Vec::with_capacity(n_out_wires as usize);
    assert_eq!(n_out_wires, 1, "MAND Gates not implemented yet!"); // TODO MAND

    for _ in 0..n_in_wires {
        in_wires.push(iter.next().unwrap().parse().unwrap());
    }
    for _ in 0..n_out_wires {
        out_wires.push(iter.next().unwrap().parse().unwrap());
    }

    let gate = match iter.next().unwrap() {
        "XOR" => Gate::XOR(in_wires[0], in_wires[1]),
        "AND" => Gate::AND(in_wires[0], in_wires[1]),
        "INV" => Gate::INV(in_wires[0]),
        "EQ" => {
            assert!((0..=1).contains(&in_wires[0]));
            Gate::EQ { constant: in_wires[0] == 1 }
        },
        "EQW" => Gate::EQW(in_wires[0]),

        // TODO: "MAND" => ...
        g => panic!("Unknown gate type {}!", g)
    };

    wires.push(Wire::Gate(gate));
}

impl Circuit {
    /// Parses the bristol file contents into a circuit
    pub fn parse(circuit: &str) -> Self {
        let mut lines = circuit.lines();

        let header = parse_header(&mut lines);

        // skip empty line
        assert_eq!(
            lines.next().expect("Expected empty line, got EOF"), "",
            "Expected empty line"
        );

        let mut wires = Vec::with_capacity(header.num_wires as usize);
        for _ in 0..header.wires_per_input.iter().sum() {
            wires.push(Wire::Input)
        }

        for _ in 0..header.num_gates {
            parse_gate(lines.next().expect("Expected gate, got EOF"), &mut wires);
        }

        Circuit { wires }
    }
}


// A `#[cfg(test)]` marks the following block as conditionally included only for test builds.
// cfg directives can achieve similar things as preprocessor directives in C/C++.
#[cfg(test)]
mod tests {
    use crate::circuit::{Circuit, Gate, Wire};

    // Functions marked with `#[test]` are automatically run when you execute `cargo test`.
    #[test]
    fn test_simple_and() {
        let source = "\
            1 3\n\
            2 1 1\n\
            1 1\n\
            \n\
            2 1 0 1 2 AND\n";

        let circuit = Circuit::parse(source);

        assert_eq!(
            circuit.wires,
            vec![Wire::Input, Wire::Input, Wire::Gate(Gate::AND(0, 1))]
        );
    }

}