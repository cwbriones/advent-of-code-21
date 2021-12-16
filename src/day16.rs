use crate::prelude::*;

fn parse(input: &str) -> &str {
    input
}

fn part_one(message: &str) -> usize {
    let msg = message.chars().filter_map(hex_to_bin).collect::<String>();
    let mut parser = Parser { msg: &msg };
    parser.packet().unwrap().version_sum()
}

fn part_two(message: &str) -> usize {
    let msg = message.chars().filter_map(hex_to_bin).collect::<String>();
    let mut parser = Parser { msg: &msg };
    parser.packet().unwrap().eval()
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}

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

struct Parser<'a> {
    msg: &'a str,
}

impl<'a> Parser<'a> {
    fn packet(&mut self) -> Result<Packet> {
        let version = self.fixint(3).context("version")?;
        let id = self.fixint(3).context("id")?;
        let kind = self.kind(id)?;
        Ok(Packet { version, kind })
    }

    fn kind(&mut self, id: usize) -> Result<PacketKind> {
        let opcode = match id {
            0 => Opcode::Sum,
            1 => Opcode::Product,
            2 => Opcode::Min,
            3 => Opcode::Max,
            5 => Opcode::GreaterThan,
            6 => Opcode::LessThan,
            7 => Opcode::Equal,
            4 => return self.varint().map(PacketKind::Literal).context("literal"),
            _ => return Err(anyhow!("unknown id: {}", id)),
        };
        self.operator(opcode).context("operator")
    }

    fn operator(&mut self, opcode: Opcode) -> Result<PacketKind> {
        let length_id = self.advance(1)?;
        if length_id == "0" {
            let total_len = self.fixint(15).context("subpacket len")?;

            let msg = self.advance(total_len)?;
            let mut subparser = Parser { msg };
            let mut subpackets = Vec::new();
            loop {
                match subparser.packet() {
                    Ok(p) => subpackets.push(p),
                    Err(_) if subparser.msg.chars().all(|c| c == '0') => break,
                    Err(e) => return Err(e).context("subpacket"),
                }
            }
            Ok(PacketKind::Operator(opcode, subpackets))
        } else {
            let num_subpackets = self.fixint(11).context("subpacket count")?;
            let subpackets = (0..num_subpackets)
                .map(|_| self.packet())
                .collect::<Result<Vec<_>, _>>()
                .context("subpacket")?;
            Ok(PacketKind::Operator(opcode, subpackets))
        }
    }

    fn fixint(&mut self, n: usize) -> Result<usize> {
        self.advance(n)
            .and_then(|bin| usize::from_str_radix(bin, 2).map_err(Into::into))
    }

    fn varint(&mut self) -> Result<usize> {
        let mut total = 0;
        loop {
            let frame = self.fixint(5)?;
            total = (total << 4) | (frame & 15);
            if (frame & (1 << 4)) == 0 {
                break;
            }
        }
        Ok(total)
    }

    fn advance(&mut self, n: usize) -> Result<&str> {
        if self.msg.len() < n {
            return Err(anyhow!("unexpected end of input"));
        }
        let (head, tail) = self.msg.split_at(n);
        self.msg = tail;
        Ok(head)
    }
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
