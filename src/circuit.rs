#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Gate {
    XOR(usize, usize),
    AND(usize, usize),
    INV(usize),
    EQ { constant: bool },
    EQW(usize),
    // TODO MAND {...},
}

#[derive(Debug, Clone)]
pub struct Header {
    pub num_gates: usize,
    pub num_wires: usize,

    pub wires_per_input: Vec<usize>,
    pub wires_per_output: Vec<usize>,
}

#[derive(Clone)]
pub struct Circuit {
    pub header: Header,
    pub gates: Vec<(Gate, usize)>,
}

fn parse_header(lines: &mut impl Iterator<Item=String>) -> Header {
    let line = lines.next().unwrap();
    let mut line_iter = line.split(' ');
    let num_gates: usize = line_iter.next().unwrap().parse().unwrap();
    let num_wires: usize = line_iter.next().unwrap().parse().unwrap();
    assert!(line_iter.next().is_none());

    let line = lines.next().unwrap();
    let mut line_iter = line.split(' ');
    let niv: usize = line_iter.next().unwrap().parse().unwrap();
    let wires_per_input: Vec<usize> = line_iter
        .take(niv)
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(wires_per_input.len(), niv);

    let line = lines.next().unwrap();
    let mut line_iter = line.split(' ');
    let nov: usize = line_iter.next().unwrap().parse().unwrap();
    let wires_per_output: Vec<usize> = line_iter
        .take(nov)
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(wires_per_output.len(), nov);

    Header { num_gates, num_wires, wires_per_input, wires_per_output }
}

fn parse_gate(gate_repr: &str, gates: &mut Vec<(Gate, usize)>) {
    let mut iter = gate_repr.split(' ');

    let n_in_wires: usize = iter.next().unwrap().parse().unwrap();
    assert!(n_in_wires == 1 || n_in_wires == 2);
    let n_out_wires: usize = iter.next().unwrap().parse().unwrap();

    let mut in_wires: Vec<usize> = Vec::with_capacity(n_in_wires);
    let mut out_wires: Vec<usize> = Vec::with_capacity(n_out_wires);
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

    gates.push((gate, out_wires[0]));
}

impl Circuit {
    /// Parses the bristol file contents into a circuit
    pub fn parse(circuit: &str) -> Self {
        return Self::parse_lines(&mut circuit.lines().map(|s| s.to_string()));
    }

    pub fn parse_lines(circuit: &mut dyn Iterator<Item=String>) -> Self {
        let mut lines = circuit;

        let header = parse_header(&mut lines);

        // skip empty line
        assert_eq!(
            lines.next().expect("Expected empty line, got EOF"), "",
            "Expected empty line"
        );

        let mut gates = Vec::with_capacity(header.num_gates);

        for _ in 0..header.num_gates {
            parse_gate(&lines.next().expect("Expected gate, got EOF"), &mut gates);
        }

        Circuit { header, gates }
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
            |w| matches!(w, (Gate::AND(_, _), _))
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

        let circuit = Circuit::parse(source);

        assert_eq!(
            circuit.gates,
            vec![(Gate::AND(0, 1), 2)]
        );
    }

}