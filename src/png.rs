use std::convert::TryFrom;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use crate::chunk::Chunk;
use anyhow::anyhow;
use crate::{Error, Result};
use crate::chunk_type::ChunkType;

pub struct Png{
    header: [u8; 8],
    chunks: Vec<Chunk>,
}

impl Png {
    // Fill in this array with the correct values per the PNG spec
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    /// Creates a `Png` from a list of chunks using the correct header
    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        Png {
            header: Png::STANDARD_HEADER,
            chunks
        }
    }

    /// Creates a `Png` from a file path
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        Png::from_str(content.as_slice())
    }

    fn from_str(content: &[u8]) -> Result<Self> {
        let (header, chunks_u8) = content.split_at(Png::STANDARD_HEADER.len());
        let header = <[u8; 8]>::try_from(header)?;
        if header != Png::STANDARD_HEADER {
            return Err(anyhow!("invalid header"));
        }
        let mut chunks = Vec::new();
        let mut pos = 0usize;
        let mut should_exit = false;
        loop {
            if should_exit || pos >= chunks_u8.len() {
                break;
            }
            let length = match <[u8; 4]>::try_from(&chunks_u8[pos..pos+4]) {
                Ok(tmp) => u32::from_be_bytes(tmp),
                Err(e) => return Err(e.into()),
            };
            let chunk_type = match <[u8; 4]>::try_from(&chunks_u8[pos+4..pos+8]) {
                Ok(tmp) => ChunkType::try_from(tmp)?,
                Err(e) => return Err(e.into()),
            };
            let chunk = Chunk::try_from(&chunks_u8[pos..pos+(length as usize)+12])?;
            chunks.push(chunk);
            pos += length as usize + 12;
        }
        Ok(Png {
            header,
            chunks,
        })
    }

    /// Appends a chunk to the end of this `Png` file's `Chunk` list.
    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.insert(self.chunks.len()-1, chunk);
    }

    /// Searches for a `Chunk` with the specified `chunk_type` and removes the first
    /// matching `Chunk` from this `Png` list of chunks.
    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        let chunk_type = ChunkType::from_str(chunk_type)?;
        let pos = self.chunks.iter()
            .position(|chunk| chunk.chunk_type() == chunk_type);
        if let Some(pos) = pos {
            return Ok(self.chunks.remove(pos));
        }
        Err(anyhow!("Chunk type not found, type = {:?} ", chunk_type))
    }

    /// The header of this PNG.
    pub fn header(&self) -> &[u8; 8] {
        &self.header
    }

    /// Lists the `Chunk`s stored in this `Png`
    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    /// Searches for a `Chunk` with the specified `chunk_type` and returns the first
    /// matching `Chunk` from this `Png`.
    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        let pos = self.chunks.iter()
            .position(|chunk| chunk.chunk_type().bytes()[..].as_ref() == chunk_type.as_bytes());
        if let Some(pos) = pos {
            return Some(&self.chunks[pos]);
        }
        None
    }

    /// Returns this `Png` as a byte sequence.
    /// These bytes will contain the header followed by the bytes of all of the chunks.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::from(self.header.clone());
        let mut chunks = self.chunks.
            iter().
            flat_map(|chunk| chunk.as_bytes()).
            collect::<Vec<u8>>();
        bytes.append(&mut chunks);
        bytes
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        Png::from_str(value)
    }
}

impl fmt::Display for Png {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_bytes())?;
        Ok(())
    }
}