use crate::cell::{Cell, CellPos, ChunkPos, LocalPos, MaterialId};
use crate::material::{MaterialKind, MaterialRegistry};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const CHUNK_SIZE: usize = 64;
pub const CHUNK_AREA: usize = CHUNK_SIZE * CHUNK_SIZE;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub pos: ChunkPos,
    pub cells: Vec<Cell>,
    pub dirty_render: bool,
    pub dirty_light: bool,
    pub dirty_save: bool,
    pub active_simulation: bool,
    pub last_simulated_tick: u64,
}

impl Chunk {
    pub fn new(pos: ChunkPos, fill: Cell) -> Self {
        Self {
            pos,
            cells: vec![fill; CHUNK_AREA],
            dirty_render: true,
            dirty_light: true,
            dirty_save: true,
            active_simulation: false,
            last_simulated_tick: 0,
        }
    }

    pub fn get(&self, local: LocalPos) -> Cell {
        self.cells[local_index(local)]
    }

    pub fn set(&mut self, local: LocalPos, cell: Cell) {
        self.cells[local_index(local)] = cell;
        self.mark_dirty();
    }

    pub fn mark_dirty(&mut self) {
        self.dirty_render = true;
        self.dirty_light = true;
        self.dirty_save = true;
        self.active_simulation = true;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub chunks: HashMap<ChunkPos, Chunk>,
    pub seed: u64,
    pub tick: u64,
    pub materials: MaterialRegistry,
}

impl World {
    pub fn new(seed: u64, materials: MaterialRegistry) -> Self {
        Self {
            chunks: HashMap::new(),
            seed,
            tick: 0,
            materials,
        }
    }

    pub fn ensure_chunk(&mut self, pos: ChunkPos) -> &mut Chunk {
        let air = Cell::new(self.materials.air());
        self.chunks
            .entry(pos)
            .or_insert_with(|| Chunk::new(pos, air))
    }

    pub fn get_cell(&self, pos: CellPos) -> Cell {
        let chunk_pos = cell_to_chunk(pos);
        let local = cell_to_local(pos);
        self.chunks
            .get(&chunk_pos)
            .map(|chunk| chunk.get(local))
            .unwrap_or_else(|| Cell::new(self.materials.air()))
    }

    pub fn set_cell(&mut self, pos: CellPos, cell: Cell) {
        let chunk_pos = cell_to_chunk(pos);
        let local = cell_to_local(pos);
        self.ensure_chunk(chunk_pos).set(local, cell);
    }

    pub fn swap_cells(&mut self, a: CellPos, b: CellPos) {
        let mut cell_a = self.get_cell(a);
        let mut cell_b = self.get_cell(b);
        cell_a.updated_tick = self.tick;
        cell_b.updated_tick = self.tick;
        self.set_cell(a, cell_b);
        self.set_cell(b, cell_a);
    }

    pub fn material_kind_at(&self, pos: CellPos) -> MaterialKind {
        self.materials
            .get(self.get_cell(pos).material)
            .map(|material| material.kind)
            .unwrap_or(MaterialKind::Empty)
    }

    pub fn is_empty(&self, pos: CellPos) -> bool {
        self.material_kind_at(pos) == MaterialKind::Empty
    }

    pub fn can_displace(&self, source: MaterialId, target: CellPos) -> bool {
        if self.is_empty(target) {
            return true;
        }

        let source_density = self
            .materials
            .get(source)
            .map(|material| material.density)
            .unwrap_or_default();
        let target_cell = self.get_cell(target);
        let target_def = self.materials.get(target_cell.material);

        matches!(
            target_def.map(|material| material.kind),
            Some(MaterialKind::Liquid)
        ) && source_density
            > target_def
                .map(|material| material.density)
                .unwrap_or_default()
    }
}

pub fn cell_to_chunk(pos: CellPos) -> ChunkPos {
    ChunkPos {
        x: pos.x.div_euclid(CHUNK_SIZE as i32),
        y: pos.y.div_euclid(CHUNK_SIZE as i32),
    }
}

pub fn cell_to_local(pos: CellPos) -> LocalPos {
    LocalPos {
        x: pos.x.rem_euclid(CHUNK_SIZE as i32) as u8,
        y: pos.y.rem_euclid(CHUNK_SIZE as i32) as u8,
    }
}

fn local_index(local: LocalPos) -> usize {
    local.y as usize * CHUNK_SIZE + local.x as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_negative_cell_positions_with_euclidean_division() {
        let pos = CellPos { x: -1, y: -65 };

        assert_eq!(cell_to_chunk(pos), ChunkPos { x: -1, y: -2 });
        assert_eq!(cell_to_local(pos), LocalPos { x: 63, y: 63 });
    }

    #[test]
    fn set_and_get_work_across_chunk_boundaries() {
        let registry = MaterialRegistry::core();
        let sand = registry.id("core:sand").unwrap();
        let mut world = World::new(42, registry);
        let pos = CellPos { x: -1, y: 64 };

        world.set_cell(pos, Cell::new(sand));

        assert_eq!(world.get_cell(pos).material, sand);
        assert_eq!(cell_to_chunk(pos), ChunkPos { x: -1, y: 1 });
    }
}
