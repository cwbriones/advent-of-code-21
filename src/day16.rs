use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
struct Packet {
    version: usize,
    kind: PacketKind,
}

impl Packet {
    fn version_sum(&self) -> usize {
        let mut total = self.version;
        if let PacketKind::Operator(_, ref subpackets) = self.kind {
            total += subpackets.iter().map(|p| p.version_sum()).sum::<usize>();
        }
        total
    }

    fn eval(&self) -> usize {
        self.kind.eval()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum PacketKind {
    Literal(usize),
    Operator(Opcode, Vec<Packet>),
}

impl PacketKind {
    fn eval(&self) -> usize {
        let (opcode, args) = match self {
            PacketKind::Literal(val) => return *val,
            PacketKind::Operator(o, p) => (o, p),
        };
        match (opcode, &args[..]) {
            (Opcode::Sum, _) => args.iter().map(|p| p.kind.eval()).sum::<usize>(),
            (Opcode::Product, _) => args.iter().map(|p| p.kind.eval()).product::<usize>(),
            (Opcode::Min, _) => args.iter().map(|p| p.kind.eval()).min().unwrap(),
            (Opcode::Max, _) => args.iter().map(|p| p.kind.eval()).max().unwrap(),
            (Opcode::GreaterThan, &[ref a, ref b]) => (a.eval() > b.eval()) as usize,
            (Opcode::LessThan, &[ref a, ref b]) => (a.eval() < b.eval()) as usize,
            (Opcode::Equal, &[ref a, ref b]) => (a.eval() == b.eval()) as usize,
            _ => panic!("invalid operation"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Opcode {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    Equal,
}

fn hex_to_bin(c: char) -> Option<&'static str> {
    let bin = match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => return None,
    };
    Some(bin)
}

fn parse(input: &str) -> &str {
    input
}

fn part_one(message: &str) -> usize {
    let out = message.chars().filter_map(hex_to_bin).collect::<String>();
    let (packet, _) = packet(&out).unwrap();
    packet.version_sum()
}

fn part_two(message: &str) -> usize {
    let out = message.chars().filter_map(hex_to_bin).collect::<String>();
    let (packet, _) = packet(&out).unwrap();
    packet.eval()
}

fn packet(msg: &str) -> Result<(Packet, &str)> {
    let (version, msg) = fixint(msg, 3).context("version")?;
    let (id, msg) = fixint(msg, 3).context("id")?;
    let (kind, msg) = match id {
        4 => literal(msg).context("literal"),
        _ if (0..=7).contains(&id) => {
            let opcode = match id {
                0 => Opcode::Sum,
                1 => Opcode::Product,
                2 => Opcode::Min,
                3 => Opcode::Max,
                5 => Opcode::GreaterThan,
                6 => Opcode::LessThan,
                7 => Opcode::Equal,
                _ => unreachable!("guarded by outer block"),
            };
            operator(opcode, msg).context("operator")
        }
        _ => return Err(anyhow!("unknown id: {}", id)),
    }?;
    Ok((Packet { version, kind }, msg))
}

fn packets(mut msg: &str) -> Result<(Vec<Packet>, &str)> {
    let mut results = Vec::new();
    while !msg.is_empty() {
        let (packet, rest) = packet(msg)?;
        msg = rest;
        results.push(packet);
    }
    Ok((results, msg))
}

fn literal(msg: &str) -> Result<(PacketKind, &str)> {
    let (val, msg) = varint(msg)?;
    Ok((PacketKind::Literal(val), msg))
}

fn operator(opcode: Opcode, msg: &str) -> Result<(PacketKind, &str)> {
    let (length_id, msg) = advance(msg, 1)?;
    if length_id == "0" {
        let (total_len, msg) = fixint(msg, 15).context("subpacket len")?;
        let (raw_subpackets, msg) = advance(msg, total_len)?;
        let (subpackets, rest) = packets(raw_subpackets).context("reading subpackets")?;
        if !rest.is_empty() {
            return Err(anyhow!("did not consume expected length of subpackets"));
        }
        Ok((PacketKind::Operator(opcode, subpackets), msg))
    } else {
        let (num_subpackets, mut msg) = fixint(msg, 11).context("subpacket count")?;
        let mut subpackets = Vec::new();
        for _ in 0..num_subpackets {
            let (packet, rest) = packet(msg).context("reading subpackets")?;
            msg = rest;
            subpackets.push(packet);
        }
        Ok((PacketKind::Operator(opcode, subpackets), msg))
    }
}

fn fixint(msg: &str, n: usize) -> Result<(usize, &str)> {
    let (raw, rest) = advance(msg, n)?;
    let val = usize::from_str_radix(raw, 2)?;
    Ok((val, rest))
}

fn varint(mut msg: &str) -> Result<(usize, &str)> {
    let mut total = 0;
    loop {
        let (frame, rest) = fixint(msg, 5)?;
        msg = rest;
        total = (total << 4) | (frame & 15);
        if (frame & (1 << 4)) == 0 {
            break;
        }
    }
    Ok((total, msg))
}

fn advance(msg: &str, n: usize) -> Result<(&str, &str)> {
    if msg.len() < n {
        return Err(anyhow!("unexpected end of input"));
    }
    Ok(msg.split_at(n))
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_literal() {
        let (packet, s) = packet("110100101111111000101000").unwrap();
        assert_eq!(
            packet,
            Packet {
                version: 6,
                kind: PacketKind::Literal(2021)
            }
        );
        assert!(s.chars().all(|c| c == '0'));
    }

    #[test]
    fn test_operator() {
        let (packet, s) =
            packet("00111000000000000110111101000101001010010001001000000000").unwrap();
        assert_eq!(
            packet,
            Packet {
                version: 1,
                kind: PacketKind::Operator(
                    Opcode::LessThan,
                    vec![
                        Packet {
                            version: 6,
                            kind: PacketKind::Literal(10)
                        },
                        Packet {
                            version: 2,
                            kind: PacketKind::Literal(20)
                        },
                    ]
                )
            }
        );
        assert!(s.chars().all(|c| c == '0'));
    }

    #[test]
    fn test_operator2() {
        let (packet, s) =
            packet("11101110000000001101010000001100100000100011000001100000").unwrap();
        assert_eq!(
            packet,
            Packet {
                version: 7,
                kind: PacketKind::Operator(
                    Opcode::Max,
                    vec![
                        Packet {
                            version: 2,
                            kind: PacketKind::Literal(1)
                        },
                        Packet {
                            version: 4,
                            kind: PacketKind::Literal(2)
                        },
                        Packet {
                            version: 1,
                            kind: PacketKind::Literal(3)
                        },
                    ]
                )
            }
        );
        assert!(s.chars().all(|c| c == '0'));
    }
}
