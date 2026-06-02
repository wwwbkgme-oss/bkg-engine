pub mod cell;
pub mod generation;
pub mod material;
pub mod save;
pub mod simulation;
pub mod world;

pub use cell::{Cell, CellPos, ChunkPos, LocalPos, MaterialId};
pub use generation::{FlatWorldGenerator, WorldGenerator};
pub use material::{MaterialDef, MaterialKind, MaterialRegistry};
pub use simulation::{SimulationConfig, SimulationStats, simulate_world};
pub use world::{CHUNK_AREA, CHUNK_SIZE, Chunk, World, cell_to_chunk, cell_to_local};
