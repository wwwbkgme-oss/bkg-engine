use crate::cell::MaterialId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterialKind {
    Empty,
    Solid,
    Powder,
    Liquid,
    Gas,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterialDef {
    pub id: MaterialId,
    pub key: String,
    pub name: String,
    pub kind: MaterialKind,
    pub color: [u8; 4],
    pub density: u16,
    pub hardness: u8,
    pub viscosity: u8,
    pub transparent: bool,
    pub collidable: bool,
    pub flammable: bool,
    pub emissive_light: [u8; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialRegistry {
    materials: Vec<MaterialDef>,
}

impl MaterialRegistry {
    pub fn core() -> Self {
        let mut registry = Self {
            materials: Vec::new(),
        };
        registry.register("core:air", "Air", MaterialKind::Empty, [0, 0, 0, 0]);
        registry.register("core:dirt", "Dirt", MaterialKind::Solid, [111, 78, 48, 255]);
        registry.register(
            "core:stone",
            "Stone",
            MaterialKind::Solid,
            [92, 95, 99, 255],
        );
        registry.register(
            "core:sand",
            "Sand",
            MaterialKind::Powder,
            [205, 178, 112, 255],
        );
        registry.register(
            "core:water",
            "Water",
            MaterialKind::Liquid,
            [47, 111, 197, 190],
        );
        registry
    }

    pub fn register(
        &mut self,
        key: impl Into<String>,
        name: impl Into<String>,
        kind: MaterialKind,
        color: [u8; 4],
    ) -> MaterialId {
        let id = MaterialId(self.materials.len() as u16);
        let (density, hardness, viscosity, transparent, collidable) = match kind {
            MaterialKind::Empty => (0, 0, 0, true, false),
            MaterialKind::Solid => (1000, 10, 0, false, true),
            MaterialKind::Powder => (900, 3, 0, false, true),
            MaterialKind::Liquid => (700, 0, 4, true, false),
            MaterialKind::Gas => (10, 0, 1, true, false),
        };
        self.materials.push(MaterialDef {
            id,
            key: key.into(),
            name: name.into(),
            kind,
            color,
            density,
            hardness,
            viscosity,
            transparent,
            collidable,
            flammable: false,
            emissive_light: [0, 0, 0],
        });
        id
    }

    pub fn get(&self, id: MaterialId) -> Option<&MaterialDef> {
        self.materials.get(id.0 as usize)
    }

    pub fn id(&self, key: &str) -> Option<MaterialId> {
        self.materials
            .iter()
            .find(|material| material.key == key)
            .map(|material| material.id)
    }

    pub fn air(&self) -> MaterialId {
        self.id("core:air").expect("core registry must contain air")
    }
}
