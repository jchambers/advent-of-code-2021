use hex::FromHexError;
use std::cmp::min;
use std::{env, error};

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

    pub fn len(&self) -> usize {
        self.bytes.len() * 8
    }

    pub fn next_bits(&mut self, n_bits: usize) -> Option<Vec<u8>> {
        if self.position + n_bits >= self.len() {
            return None;
        }

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

        Some(collected_bytes)
    }

    fn next_bit(&mut self) -> u8 {
        let byte_index = self.position / 8;
        let bit_offset = self.position % 8;

        self.position += 1;

        (self.bytes[byte_index] & BitStream::BIT_MASK[bit_offset]) >> (7 - bit_offset)
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

        assert_eq!(Some(vec![0b110]), bit_stream.next_bits(3));
        assert_eq!(Some(vec![0b100]), bit_stream.next_bits(3));
        assert_eq!(Some(vec![0b10111]), bit_stream.next_bits(5));
        assert_eq!(Some(vec![0b11110]), bit_stream.next_bits(5));
        assert_eq!(Some(vec![0b00101]), bit_stream.next_bits(5));
    }
}
