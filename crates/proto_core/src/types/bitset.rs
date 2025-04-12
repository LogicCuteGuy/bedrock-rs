use std::io::{Cursor, Read};
use std::convert::TryInto;
use crate::error::ProtoCodecError;
use crate::ProtoCodec;

#[derive(Debug, Clone, PartialEq)]
pub struct BitSet<const N: usize> {
    storage: Vec<u64>,
}

impl<const N: usize> BitSet<N> {
    const CHUNKS: usize = (N + 63) / 64;

    pub fn new() -> Self {
        Self {
            storage: vec![0; Self::CHUNKS],
        }
    }

    pub fn set(&mut self, index: usize, value: bool) {
        assert!(index < N, "Index out of bounds");
        let (chunk, bit) = (index / 64, index % 64);
        if value {
            self.storage[chunk] |= 1 << bit;
        } else {
            self.storage[chunk] &= !(1 << bit);
        }
    }

    pub fn get(&self, index: usize) -> bool {
        assert!(index < N, "Index out of bounds");
        let (chunk, bit) = (index / 64, index % 64);
        (self.storage[chunk] >> bit) & 1 != 0
    }
}

impl<const N: usize> ProtoCodec for BitSet<N> {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        for chunk in &self.storage {
            stream.extend_from_slice(&chunk.to_le_bytes());
        }
        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        let mut storage = Vec::with_capacity(Self::CHUNKS);
        let mut buffer = [0u8; 8];

        for _ in 0..Self::CHUNKS {
            stream.read_exact(&mut buffer)?;
            storage.push(u64::from_le_bytes(buffer));
        }

        // Mask out unused bits in last chunk
        if N % 64 != 0 {
            if let Some(last) = storage.last_mut() {
                *last &= (1 << (N % 64)) - 1;
            }
        }

        Ok(Self { storage })
    }

    fn get_size_prediction(&self) -> usize {
        Self::CHUNKS * 8  // 8 bytes per u64
    }
}