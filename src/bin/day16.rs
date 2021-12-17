use crate::Packet::Operator;
use hex::FromHexError;
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let mut bit_stream = BitStream::from_hex(std::fs::read_to_string(path)?.as_str())?;
        let packet = Packet::next_from_bit_stream(&mut bit_stream);

        println!("Version sum: {}", packet.version_sum());

        Ok(())
    } else {
        Err("Usage: day16 INPUT_FILE_PATH".into())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Header {
    version: u8,
    type_id: u8,
}

impl Header {
    pub fn next_from_bit_stream(bit_stream: &mut BitStream) -> Self {
        Header {
            version: bit_stream.next_bits(3) as u8,
            type_id: bit_stream.next_bits(3) as u8,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Literal {
        header: Header,
        value: u64,
    },

    Operator {
        header: Header,
        sub_packets: Vec<Packet>,
    },
}

impl Packet {
    const LITERAL_TYPE_ID: u8 = 4;
    const LITERAL_HAS_MORE_BIT: u8 = 0b00010000;
    const LITERAL_NIBBLE_MASK: u8 = 0b00001111;

    fn from_hex(hex: &str) -> Self {
        let mut bit_stream = BitStream::from_hex(hex).unwrap();

        Packet::next_from_bit_stream(&mut bit_stream)
    }

    fn next_from_bit_stream(bit_stream: &mut BitStream) -> Self {
        let header = Header::next_from_bit_stream(bit_stream);

        match header.type_id {
            Packet::LITERAL_TYPE_ID => {
                // Literal! Read five-bit chunks until we get a "last chunk" bit.
                let mut value = 0u64;

                loop {
                    let next_nibble = bit_stream.next_bits(5) as u8;

                    value <<= 4;
                    value |= (next_nibble & Packet::LITERAL_NIBBLE_MASK) as u64;

                    if next_nibble & Packet::LITERAL_HAS_MORE_BIT == 0 {
                        break;
                    }
                }

                Packet::Literal { header, value }
            }
            _ => {
                // Operator
                let length_type = bit_stream.next_bits(1);

                let mut sub_packets = Vec::new();

                if length_type == 0 {
                    // 15-bit bit count
                    let target_bit_count = bit_stream.next_bits(15) as usize;
                    let target_position = bit_stream.position() + target_bit_count;

                    while bit_stream.position() < target_position {
                        sub_packets.push(Packet::next_from_bit_stream(bit_stream));
                    }
                } else {
                    // 11-bit packet count
                    let target_packet_count = bit_stream.next_bits(11);

                    for _ in 0..target_packet_count {
                        sub_packets.push(Packet::next_from_bit_stream(bit_stream));
                    }
                }

                Operator {
                    header,
                    sub_packets,
                }
            }
        }
    }

    pub fn version_sum(&self) -> u32 {
        match self {
            Packet::Literal { header, value: _ } => header.version as u32,
            Packet::Operator {
                header,
                sub_packets,
            } => {
                header.version as u32
                    + sub_packets
                        .iter()
                        .map(|sub_packet| sub_packet.version_sum())
                        .sum::<u32>()
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct BitStream {
    bytes: Vec<u8>,
    position: usize,
}

impl BitStream {
    const BIT_MASK: [u8; 8] = [
        0b10000000,
        0b01000000,
        0b00100000,
        0b00010000,
        0b00001000,
        0b00000100,
        0b00000010,
        0b00000001,
    ];

    pub fn from_hex(hex_string: &str) -> Result<Self, FromHexError> {
        Ok(BitStream::new(hex::decode(hex_string)?))
    }

    pub fn new(bytes: Vec<u8>) -> Self {
        BitStream { bytes, position: 0 }
    }

    pub fn next_bits(&mut self, n_bits: usize) -> u32 {
        let mut next_bits = 0u32;

        for _ in 0..n_bits {
            next_bits <<= 1;
            next_bits |= self.next_bit() as u32;
        }

        next_bits
    }

    fn next_bit(&mut self) -> u8 {
        let byte_index = self.position / 8;
        let bit_offset = self.position % 8;

        self.position += 1;

        (self.bytes[byte_index] & BitStream::BIT_MASK[bit_offset]) >> (7 - bit_offset)
    }

    pub fn position(&self) -> usize {
        self.position
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Packet::Literal;
    use std::collections::VecDeque;

    #[test]
    fn test_bit_stream_from_hex() {
        assert_eq!(
            BitStream::new(vec![0xd2, 0xfe, 0x28]),
            BitStream::from_hex("D2FE28").unwrap()
        );
    }

    #[test]
    fn test_get_bit() {
        let mut bit_stream = BitStream::new(vec![0xd2, 0xfe, 0x28]);
        let mut expected = VecDeque::from([
            1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0,
        ]);

        while let Some(expected_bit) = expected.pop_front() {
            assert_eq!(expected_bit, bit_stream.next_bit());
        }
    }

    #[test]
    fn test_next_bits() {
        let mut bit_stream = BitStream::new(vec![0xd2, 0xfe, 0x28]);

        assert_eq!(0b110, bit_stream.next_bits(3));
        assert_eq!(0b100, bit_stream.next_bits(3));
        assert_eq!(0b10111, bit_stream.next_bits(5));
        assert_eq!(0b11110, bit_stream.next_bits(5));
        assert_eq!(0b00101, bit_stream.next_bits(5));
    }

    #[test]
    fn test_literal_from_bit_stream() {
        assert_eq!(
            Packet::Literal {
                header: Header {
                    version: 6,
                    type_id: 4,
                },
                value: 2021
            },
            Packet::from_hex("D2FE28")
        );
    }

    #[test]
    fn test_operator_from_bit_count() {
        let expected = Packet::Operator {
            header: Header {
                version: 1,
                type_id: 6,
            },
            sub_packets: vec![
                Literal {
                    header: Header {
                        version: 6,
                        type_id: 4,
                    },
                    value: 10,
                },
                Literal {
                    header: Header {
                        version: 2,
                        type_id: 4,
                    },
                    value: 20,
                },
            ],
        };

        assert_eq!(expected, Packet::from_hex("38006F45291200"));
    }

    #[test]
    fn test_operator_from_packet_count() {
        let expected = Packet::Operator {
            header: Header {
                version: 7,
                type_id: 3,
            },
            sub_packets: vec![
                Literal {
                    header: Header {
                        version: 2,
                        type_id: 4,
                    },
                    value: 1,
                },
                Literal {
                    header: Header {
                        version: 4,
                        type_id: 4,
                    },
                    value: 2,
                },
                Literal {
                    header: Header {
                        version: 1,
                        type_id: 4,
                    },
                    value: 3,
                },
            ],
        };

        assert_eq!(expected, Packet::from_hex("EE00D40C823060"));
    }

    #[test]
    fn test_packet_version_sum() {
        assert_eq!(16, Packet::from_hex("8A004A801A8002F478").version_sum());
        assert_eq!(12, Packet::from_hex("620080001611562C8802118E34").version_sum());
        assert_eq!(23, Packet::from_hex("C0015000016115A2E0802F182340").version_sum());
        assert_eq!(31, Packet::from_hex("A0016C880162017C3686B18A3D4780").version_sum());
    }
}
