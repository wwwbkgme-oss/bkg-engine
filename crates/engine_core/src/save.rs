use crate::world::World;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldHeader {
    pub version: u32,
    pub seed: u64,
    pub tick: u64,
    pub generator_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSave {
    pub header: WorldHeader,
    pub world: World,
}

pub fn save_world(path: impl AsRef<Path>, world: &World) -> io::Result<()> {
    let save = WorldSave {
        header: WorldHeader {
            version: 1,
            seed: world.seed,
            tick: world.tick,
            generator_id: "core:flat".to_string(),
        },
        world: world.clone(),
    };
    let bytes =
        bincode::serialize(&save).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
    fs::write(path, bytes)
}

pub fn load_world(path: impl AsRef<Path>) -> io::Result<WorldSave> {
    let bytes = fs::read(path)?;
    bincode::deserialize(&bytes).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
}
