use crate::cell::{Cell, ChunkPos};
use crate::world::{CHUNK_SIZE, Chunk};

pub trait WorldGenerator {
    fn generate_chunk(&self, pos: ChunkPos, chunk: &mut Chunk);
}

#[derive(Debug, Clone, Copy)]
pub struct FlatWorldGenerator {
    pub surface_y: i32,
}

impl WorldGenerator for FlatWorldGenerator {
    fn generate_chunk(&self, _pos: ChunkPos, chunk: &mut Chunk) {
        let air = Cell::new(crate::cell::MaterialId(0));
        let dirt = Cell::new(crate::cell::MaterialId(1));
        let stone = Cell::new(crate::cell::MaterialId(2));

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let world_y = chunk.pos.y * CHUNK_SIZE as i32 + y as i32;
                let cell = if world_y > self.surface_y {
                    air
                } else if world_y > self.surface_y - 8 {
                    dirt
                } else {
                    stone
                };
                chunk.cells[y * CHUNK_SIZE + x] = cell;
            }
        }
        chunk.mark_dirty();
    }
}
