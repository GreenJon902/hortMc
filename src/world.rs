use std::collections::HashMap;

#[derive(Debug)]
pub struct World {
    pub chunks: HashMap<i64, Chunk>
}

#[derive(Debug)]
pub struct Chunk {
    pub sections: HashMap<i8, ChunkSection>
}

#[derive(Debug)]
pub struct ChunkSection {
    pub block_count: u16,
    pub pallet: Vec<i32>,  // Stores indexes of global pallet
    pub data: [i32; 4096]
}