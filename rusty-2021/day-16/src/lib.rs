use std::collections::VecDeque;

static BITS: [&str; 16] = [
    "0000", "0001", "0010", "0011", "0100", "0101", "0110", "0111", "1000", "1001", "1010", "1011",
    "1100", "1101", "1110", "1111",
];

enum Inst {
    Literal(usize, usize),
    Operator(usize, u8, Vec<Inst>),
}

fn find_version_sum(input: &str) -> usize {
    let mut bits = parse_bits(input);
    let inst = parse_inst(&mut bits);
    version_sum(&inst)
}

fn find_value(input: &str) -> usize {
    let mut bits = parse_bits(input);
    let inst = parse_inst(&mut bits);
    value(&inst)
}

fn consume_bits(bits: &mut VecDeque<u8>, n: usize) -> usize {
    bits.drain(0..n)
        .fold(0, |acc, bit| (acc << 1) | bit as usize)
}

fn parse_inst(bits: &mut VecDeque<u8>) -> Inst {
    let version = consume_bits(bits, 3);
    let type_id = consume_bits(bits, 3) as u8;

    match type_id {
        4 => {
            let mut value = 0;
            loop {
                let group = consume_bits(bits, 5);
                value = (value << 4) | (group & 0xF);
                if group >> 4 == 0 {
                    break;
                }
            }
            Inst::Literal(version, value)
        }
        _ => {
            let length_type_id = consume_bits(bits, 1);
            let sub_insts = if length_type_id == 0 {
                let total_length = consume_bits(bits, 15);
                let mut sub_bits = bits.drain(0..total_length).collect::<VecDeque<_>>();
                let mut insts = Vec::new();
                while !sub_bits.is_empty() {
                    insts.push(parse_inst(&mut sub_bits));
                }
                insts
            } else {
                let num_sub_packets = consume_bits(bits, 11);
                (0..num_sub_packets).map(|_| parse_inst(bits)).collect()
            };
            Inst::Operator(version, type_id, sub_insts)
        }
    }
}

fn version_sum(inst: &Inst) -> usize {
    match inst {
        Inst::Literal(v, _) => *v,
        Inst::Operator(v, _, insts) => *v + insts.iter().map(version_sum).sum::<usize>(),
    }
}

fn value(inst: &Inst) -> usize {
    match inst {
        Inst::Literal(_, val) => *val as usize,
        Inst::Operator(_, id, insts) => match id {
            0 => insts.iter().map(value).sum(),
            1 => insts.iter().map(value).product(),
            2 => insts.iter().map(value).min().unwrap(),
            3 => insts.iter().map(value).max().unwrap(),
            5 => (value(&insts[0]) > value(&insts[1])) as usize,
            6 => (value(&insts[0]) < value(&insts[1])) as usize,
            7 => (value(&insts[0]) == value(&insts[1])) as usize,
            _ => unreachable!(),
        },
    }
}

fn parse_bits(input: &str) -> VecDeque<u8> {
    input
        .bytes()
        .map(|b| {
            if b >= b'A' {
                BITS[(b - b'A') as usize + 10]
            } else {
                BITS[(b - b'0') as usize]
            }
        })
        .flat_map(|bits| bits.bytes().map(|b| b - b'0'))
        .collect()
}

mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("./example_input.txt").unwrap();
        let input = fs::read_to_string("./input.txt").unwrap();

        let res1_example = find_version_sum(&example_input);
        let res1 = find_version_sum(&input);
        assert_eq!(6, res1_example);
        assert_eq!(897, res1);

        let res2_example = find_value(&example_input);
        let res2 = find_value(&input);
        assert_eq!(res2_example, 2021);
        assert_eq!(res2, 9485076995911);
    }
}
