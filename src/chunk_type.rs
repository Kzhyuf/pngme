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