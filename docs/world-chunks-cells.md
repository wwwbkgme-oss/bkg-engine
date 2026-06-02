# Welt, Chunks und Zellen

## Weltstruktur

Eine grosse Sandbox-Welt darf nicht komplett aktiv simuliert oder gerendert
werden. Deshalb wird sie in Chunks organisiert.

Empfohlener Startwert:

```text
64 x 64 Zellen pro Chunk
```

Dieser Wert ist ein guter Kompromiss zwischen Update-Kosten, Verwaltungsaufwand
und Rendering.

## Grundtypen

```rust
pub const CHUNK_SIZE: usize = 64;
pub const CHUNK_AREA: usize = CHUNK_SIZE * CHUNK_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct LocalPos {
    pub x: u8,
    pub y: u8,
}
```

## Chunk

```rust
pub struct Chunk {
    pub pos: ChunkPos,
    pub cells: Box<[Cell; CHUNK_AREA]>,

    pub dirty_render: bool,
    pub dirty_light: bool,
    pub dirty_save: bool,
    pub active_simulation: bool,

    pub last_simulated_tick: u64,
}
```

## World

```rust
use std::collections::HashMap;

pub struct World {
    pub chunks: HashMap<ChunkPos, Chunk>,
    pub seed: u64,
    pub tick: u64,
    pub materials: MaterialRegistry,
}
```

## Negative Koordinaten

Bei Weltkoordinaten muessen negative Positionen korrekt auf Chunks und lokale
Chunk-Positionen abgebildet werden.

```rust
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
```

## Zellmodell

Minimal:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub material: MaterialId,
}
```

Erweitert:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub material: MaterialId,
    pub amount: u8,
    pub temperature: i16,
    pub variant: u8,
    pub flags: CellFlags,
}
```

## MaterialId

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialId(pub u16);
```

## CellFlags

```rust
bitflags::bitflags! {
    pub struct CellFlags: u16 {
        const UPDATED_THIS_TICK = 0b0000_0001;
        const ACTIVE            = 0b0000_0010;
        const SOLID             = 0b0000_0100;
        const LIQUID            = 0b0000_1000;
        const POWDER            = 0b0001_0000;
        const EMISSIVE          = 0b0010_0000;
    }
}
```

## Umsetzungsschritte

1. `CellPos`, `ChunkPos` und `LocalPos` definieren.
2. `Chunk` mit `64x64` Zellen bauen.
3. `World` mit `HashMap<ChunkPos, Chunk>` bauen.
4. `get_cell` und `set_cell` implementieren.
5. Negative Koordinaten testen.
6. Dirty-Flags einfuehren.
7. Spaeter Chunk-Streaming ergaenzen.
