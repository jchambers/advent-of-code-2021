use hex::FromHexError;
use std::cmp::min;
use std::{env, error};
use crate::Packet::Operator;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let _bit_stream = BitStream::from_hex(std::fs::read_to_string(path)?.as_str())?;

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
            version: bit_stream.next_bits(3)[0],
            type_id: bit_stream.next_bits(3)[0],
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
    }
}

impl Packet {
    const LITERAL_TYPE_ID: u8 = 4;
    const LITERAL_HAS_MORE_BIT: u8 = 0b00010000;
    const LITERAL_NIBBLE_MASK: u8 = 0b00001111;

    pub fn next_from_bit_stream(bit_stream: &mut BitStream) -> Self {
        let header = Header::next_from_bit_stream(bit_stream);

        match header.type_id {
            Packet::LITERAL_TYPE_ID => {
                // Literal! Read five-bit chunks until we get a "last chunk" bit.
                let mut value = 0u64;

                loop {
                    let next_nibble = bit_stream.next_bits(5)[0];

                    value <<= 4;
                    value |= (next_nibble & Packet::LITERAL_NIBBLE_MASK) as u64;

                    if next_nibble & Packet::LITERAL_HAS_MORE_BIT == 0 {
                        bit_stream.align_to_nibble();
                        break;
                    }
                }

                Packet::Literal { header, value }
            },
            _ => {
                // Operator
                Operator {
                    header,
                    sub_packets: vec![]
                }
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

    pub fn next_bits(&mut self, n_bits: usize) -> Vec<u8> {
        let mut collected_bytes = Vec::new();
        let mut collected_bits = 0;

        while collected_bits < n_bits {
            let mut byte = 0;

            for _ in 0..min(n_bits - collected_bits, 8) {
                byte <<= 1;
                byte |= self.next_bit();

                collected_bits += 1;
            }

            collected_bytes.push(byte);
        }

        collected_bytes
    }

    fn next_bit(&mut self) -> u8 {
        let byte_index = self.position / 8;
        let bit_offset = self.position % 8;

        self.position += 1;

        (self.bytes[byte_index] & BitStream::BIT_MASK[bit_offset]) >> (7 - bit_offset)
    }

    pub fn align_to_nibble(&mut self) {
        self.position += (4 - (self.position % 4)) % 4;
    }
}

#[cfg(test)]
mod test {
    use super::*;
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

        assert_eq!(vec![0b110], bit_stream.next_bits(3));
        assert_eq!(vec![0b100], bit_stream.next_bits(3));
        assert_eq!(vec![0b10111], bit_stream.next_bits(5));
        assert_eq!(vec![0b11110], bit_stream.next_bits(5));
        assert_eq!(vec![0b00101], bit_stream.next_bits(5));
    }

    #[test]
    fn test_literal_from_bit_stream() {
        let mut bit_stream = BitStream::new(vec![0xd2, 0xfe, 0x28]);

        assert_eq!(Packet::Literal {
            header: Header {
                version: 6,
                type_id: 4,
            },
            value: 2021
        }, Packet::next_from_bit_stream(&mut bit_stream));
    }
}
