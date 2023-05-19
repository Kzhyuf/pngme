use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{BufReader, Read};
use crc::{Crc, CRC_32_ISO_HDLC};
use anyhow::anyhow;

use crate::{Error, Result};
use crate::chunk_type::ChunkType;

const CRC32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let mut digest = CRC32.digest_with_initial(u32::MAX);
        digest.update(chunk_type.bytes().as_ref());
        digest.update(data.as_slice());

        Chunk {
            length: data.len() as u32,
            chunk_type,
            data,
            crc: digest.finalize(),
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> ChunkType {
        self.chunk_type.clone()
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_ref()
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        match String::from_utf8(self.data.clone()) {
            Ok(str) => Ok(str),
            Err(e) => Err(e.into()),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.append(&mut self.length.to_be_bytes().to_vec());
        bytes.append(&mut self.chunk_type.bytes().to_vec());
        bytes.append(&mut self.data.clone());
        bytes.append(&mut self.crc.to_be_bytes().to_vec());
        bytes
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let length = match <[u8; 4]>::try_from(&value[..4]) {
            Ok(val) => u32::from_be_bytes(val),
            Err(e) => return Err(e.into()),
        } as usize;
        let chunk_type = match <[u8; 4]>::try_from(&value[4..8]) {
            Ok(val) => ChunkType::try_from(val)?,
            Err(e) => return Err(e.into()),
        };
        let crc = match <[u8;4]>::try_from(&value[8+length..]) {
            Ok(v) => u32::from_be_bytes(v),
            Err(_) => return Err(anyhow!("invalid crc length")),
        };
        let chunk = Chunk::new(chunk_type, value[8..8+length].to_vec());
        if chunk.crc() != crc {
            return Err(anyhow!("crc mismatched"));
        }
        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}