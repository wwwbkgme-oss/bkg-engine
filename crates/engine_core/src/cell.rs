use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MaterialId(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocalPos {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cell {
    pub material: MaterialId,
    pub amount: u8,
    pub temperature: i16,
    pub variant: u8,
    pub updated_tick: u64,
}

impl Cell {
    pub const fn new(material: MaterialId) -> Self {
        Self {
            material,
            amount: 0,
            temperature: 0,
            variant: 0,
            updated_tick: 0,
        }
    }

    pub const fn liquid(material: MaterialId, amount: u8) -> Self {
        Self {
            material,
            amount,
            temperature: 0,
            variant: 0,
            updated_tick: 0,
        }
    }
}
