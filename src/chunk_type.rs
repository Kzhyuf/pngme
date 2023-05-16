use std::convert::TryFrom;
use std::fmt::{self, Formatter};
use std::io::Stderr;
use std::str::FromStr;
use anyhow::anyhow;

use crate::{Error, Result};
/*
Chunk type structure spec
http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType([u8; 4]);


const MASK: u8 = 32;
// const Ancillary_MASK: u32 = 1<<5;
// const Private_MASK: u32 = 1<<13;
// const Reserved_MASK: u32 = 1<<21;
// const Safe_To_Copy_MASK: u32 = 1<<29;

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        self.0.iter()
            .all(|ch| ch.is_ascii_alphabetic())
        && self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        self.0[0] & MASK == 0
    }

    pub fn is_public(&self) -> bool {
        self.0[1] & MASK == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.0[2] & MASK == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.0[3] & MASK != 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        Ok(ChunkType(value))
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match <[u8; 4]>::try_from(s.as_bytes()) {
            Ok(val) => {
                let chunk = ChunkType(val);
                if chunk.0.iter().any(|&ch| !ch.is_ascii_alphabetic()) {
                    return Err(anyhow!("{} is not valid chunk type", &s));
                }
                Ok(chunk)
            },
            Err(err) => Err(err.into()),
        }
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(Vec::from(self.0)).expect("fmt error for ChunkType"))
    }
}

/*
test for chunk type
 */
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
