mod packet {
    pub struct Literal {
        pub version: u8,
        pub value: usize,
        pub size: usize,
    }
    pub struct Operator {
        pub version: u8,
        pub opcode: u8,
        pub subpackets: Vec<Packet>,
        pub size: usize,
    }
    pub enum Packet {
        Literal(Literal),
        Operator(Operator),
    }
    impl Packet {
        pub fn get_size(&self) -> usize {
            match self {
                Packet::Literal(literal) => literal.size,
                Packet::Operator(operator) => operator.size,
            }
        }
        pub fn total_version(&self) -> usize {
            match self {
                Packet::Literal(literal) => literal.version as usize,
                Packet::Operator(operator) => {
                    operator.version as usize
                        + operator
                            .subpackets
                            .iter()
                            .map(|p| p.total_version())
                            .sum::<usize>()
                }
            }
        }
        pub fn evaluate(&self) -> usize {
            match self {
                Packet::Literal(literal) => literal.value,
                Packet::Operator(operator) => {
                    let mut subpacket_values = operator.subpackets.iter().map(|p| p.evaluate());
                    match operator.opcode {
                        0 => subpacket_values.sum(),
                        1 => subpacket_values.product(),
                        2 => subpacket_values.min().unwrap(),
                        3 => subpacket_values.max().unwrap(),
                        _ => {
                            let first = subpacket_values.next().unwrap();
                            let second = subpacket_values.next().unwrap();
                            match operator.opcode {
                                5 => {
                                    if first > second {
                                        1
                                    } else {
                                        0
                                    }
                                }
                                6 => {
                                    if first < second {
                                        1
                                    } else {
                                        0
                                    }
                                }
                                7 => {
                                    if first == second {
                                        1
                                    } else {
                                        0
                                    }
                                }
                                _ => panic!("Invalid opcode"),
                            }
                        }
                    }
                }
            }
        }
    }
}

use packet::*;

pub fn parse_literal(bits: &mut std::str::Chars, version: u8) -> Literal {
    let mut has_more = true;
    let mut val = String::new();
    let mut chomped = 0;
    while has_more {
        has_more = bits.next().unwrap() == '1';
        val += &bits.take(4).collect::<String>();
        chomped += 5;
    }
    Literal {
        version,
        value: usize::from_str_radix(&val, 2).unwrap(),
        size: chomped + 6,
    }
}

pub fn parse_operator(bits: &mut std::str::Chars, version: u8, opcode: u8) -> Operator {
    let length_type = bits.next().unwrap();
    let mut packets: Vec<Packet> = vec![];
    let mut total_chomped = 7;
    if length_type == '0' {
        // next 15 bits is length
        let mut length = usize::from_str_radix(&bits.take(15).collect::<String>(), 2).unwrap();
        total_chomped += 15;
        while length != 0 {
            let packet = parse_packet(bits);
            let size = packet.get_size();
            length -= size;
            total_chomped += packet.get_size();
            packets.push(packet);
        }
    } else {
        // next 11 bits is number of packets
        let mut length = usize::from_str_radix(&bits.take(11).collect::<String>(), 2).unwrap();
        total_chomped += 11;
        while length != 0 {
            let packet = parse_packet(bits);
            length -= 1;
            total_chomped += packet.get_size();
            packets.push(packet);
        }
    }
    Operator {
        version,
        subpackets: packets,
        opcode,
        size: total_chomped,
    }
}

pub fn parse_packet(bits: &mut std::str::Chars) -> Packet {
    let version = u8::from_str_radix(&bits.take(3).collect::<String>(), 2).unwrap();
    let type_id = u8::from_str_radix(&bits.take(3).collect::<String>(), 2).unwrap();
    if type_id == 4 {
        Packet::Literal(parse_literal(bits, version))
    } else {
        Packet::Operator(parse_operator(bits, version, type_id))
    }
}

pub fn part1(raw: &str) -> usize {
    parse_packet(&mut raw.chars()).total_version()
}
pub fn part2(raw: &str) -> usize {
    parse_packet(&mut raw.chars()).evaluate()
}


pub fn day16() {
    let raw = std::fs::read_to_string("./inputs/day16.txt").unwrap();
    println!("{}", part1(&raw));
    println!("{}", part2(&raw));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("100010100000000001001010100000000001101010000000000000101111010001111000"),
            16
        );
        assert_eq!(part1("01100010000000001000000000000000000101100001000101010110001011001000100000000010000100011000111000110100"), 12);
        assert_eq!(part1("1100000000000001010100000000000000000001011000010001010110100010111000001000000000101111000110000010001101000000"), 23);
        assert_eq!(part1("101000000000000101101100100010000000000101100010000000010111110000110110100001101011000110001010001111010100011110000000"), 31);
    }
    fn test_part2() {

    }
}
