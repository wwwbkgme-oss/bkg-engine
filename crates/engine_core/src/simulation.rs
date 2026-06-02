use crate::cell::{CellPos, ChunkPos};
use crate::material::MaterialKind;
use crate::world::{CHUNK_SIZE, World};

#[derive(Debug, Clone, Copy)]
pub struct SimulationConfig {
    pub min_chunk: ChunkPos,
    pub max_chunk: ChunkPos,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SimulationStats {
    pub moved_cells: u32,
}

pub fn simulate_world(world: &mut World, config: SimulationConfig) -> SimulationStats {
    world.tick += 1;
    let mut stats = SimulationStats::default();
    let left_to_right = world.tick % 2 == 0;

    for chunk_y in config.min_chunk.y..=config.max_chunk.y {
        for local_y in 0..CHUNK_SIZE {
            let y = chunk_y * CHUNK_SIZE as i32 + local_y as i32;
            for chunk_x in config.min_chunk.x..=config.max_chunk.x {
                if left_to_right {
                    for local_x in 0..CHUNK_SIZE {
                        let x = chunk_x * CHUNK_SIZE as i32 + local_x as i32;
                        stats.moved_cells += simulate_cell(world, CellPos { x, y }) as u32;
                    }
                } else {
                    for local_x in (0..CHUNK_SIZE).rev() {
                        let x = chunk_x * CHUNK_SIZE as i32 + local_x as i32;
                        stats.moved_cells += simulate_cell(world, CellPos { x, y }) as u32;
                    }
                }
            }
        }
    }

    stats
}

fn simulate_cell(world: &mut World, pos: CellPos) -> bool {
    let cell = world.get_cell(pos);
    if cell.updated_tick == world.tick {
        return false;
    }

    match world.material_kind_at(pos) {
        MaterialKind::Powder => simulate_powder(world, pos),
        MaterialKind::Liquid => simulate_liquid(world, pos),
        _ => false,
    }
}

fn simulate_powder(world: &mut World, pos: CellPos) -> bool {
    let material = world.get_cell(pos).material;
    let below = CellPos {
        x: pos.x,
        y: pos.y - 1,
    };
    let down_left = CellPos {
        x: pos.x - 1,
        y: pos.y - 1,
    };
    let down_right = CellPos {
        x: pos.x + 1,
        y: pos.y - 1,
    };
    let diagonals = if world.tick % 2 == 0 {
        [down_left, down_right]
    } else {
        [down_right, down_left]
    };

    if world.can_displace(material, below) {
        world.swap_cells(pos, below);
        return true;
    }

    for target in diagonals {
        if world.can_displace(material, target) {
            world.swap_cells(pos, target);
            return true;
        }
    }

    false
}

fn simulate_liquid(world: &mut World, pos: CellPos) -> bool {
    let below = CellPos {
        x: pos.x,
        y: pos.y - 1,
    };
    if world.is_empty(below) {
        world.swap_cells(pos, below);
        return true;
    }

    let sideways = if world.tick % 2 == 0 {
        [
            CellPos {
                x: pos.x - 1,
                y: pos.y,
            },
            CellPos {
                x: pos.x + 1,
                y: pos.y,
            },
        ]
    } else {
        [
            CellPos {
                x: pos.x + 1,
                y: pos.y,
            },
            CellPos {
                x: pos.x - 1,
                y: pos.y,
            },
        ]
    };

    for target in sideways {
        if world.is_empty(target) {
            world.swap_cells(pos, target);
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Cell, MaterialRegistry};

    #[test]
    fn sand_falls_down_one_cell() {
        let registry = MaterialRegistry::core();
        let sand = registry.id("core:sand").unwrap();
        let mut world = World::new(7, registry);
        world.set_cell(CellPos { x: 0, y: 5 }, Cell::new(sand));

        let stats = simulate_world(
            &mut world,
            SimulationConfig {
                min_chunk: ChunkPos { x: 0, y: 0 },
                max_chunk: ChunkPos { x: 0, y: 0 },
            },
        );

        assert_eq!(stats.moved_cells, 1);
        assert!(world.is_empty(CellPos { x: 0, y: 5 }));
        assert_eq!(world.get_cell(CellPos { x: 0, y: 4 }).material, sand);
    }
}
