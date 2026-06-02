# Projektstruktur und Roadmap

## Rust Workspace

```text
voxel_engine/
+-- Cargo.toml
+-- crates/
    +-- engine_core/
    |   +-- src/
    |       +-- lib.rs
    |       +-- world/
    |       +-- chunk/
    |       +-- cell/
    |       +-- material/
    |       +-- simulation/
    |       +-- physics/
    |       +-- ecs/
    |       +-- items/
    |       +-- generation/
    |       +-- lighting/
    |       +-- save/
    |
    +-- engine_render/
    |   +-- src/
    |       +-- lib.rs
    |       +-- renderer.rs
    |       +-- camera.rs
    |       +-- chunk_renderer.rs
    |       +-- entity_renderer.rs
    |       +-- lighting_renderer.rs
    |
    +-- engine_app/
    |   +-- src/
    |       +-- lib.rs
    |       +-- input.rs
    |       +-- window.rs
    |       +-- game_loop.rs
    |
    +-- sandbox_game/
        +-- src/
            +-- main.rs
            +-- materials.rs
            +-- items.rs
            +-- enemies.rs
            +-- gameplay.rs
```

## Kernregel

```text
engine_core kennt keinen Renderer.
engine_core kennt kein Fenster.
engine_render liest Daten, veraendert aber keine Simulation.
sandbox_game registriert konkrete Inhalte.
```

## Wichtige Traits

```rust
pub trait WorldGenerator {
    fn generate_chunk(&self, pos: ChunkPos, chunk: &mut Chunk);
}

pub trait SaveBackend {
    fn save_chunk(&mut self, chunk: &Chunk) -> Result<(), SaveError>;
    fn load_chunk(&mut self, pos: ChunkPos) -> Result<Option<Chunk>, SaveError>;
}

pub trait SimulationSystem {
    fn update(&mut self, world: &mut World, dt: f32);
}

pub trait PhysicsWorldQuery {
    fn is_solid(&self, pos: CellPos) -> bool;
    fn material_at(&self, pos: CellPos) -> MaterialId;
}

pub trait WorldRenderer {
    fn update_chunk(&mut self, chunk: &Chunk);
    fn render_world(&mut self, camera: &Camera2D);
}
```

Nur `WorldGenerator`, `SaveBackend` und `Renderer` sollten frueh abstrahiert
werden. Simulation erst konkret implementieren.

## Roadmap

### Meilenstein 1: Minimal-Prototyp

- Fenster
- feste Weltgroesse, zum Beispiel `256x256`
- Materialien: Luft, Erde, Stein
- einfache Kamera
- Zellrendering
- Maus setzt und loescht Bloecke

### Meilenstein 2: Chunk-System

- `Chunk`
- `ChunkPos`
- `CellPos`
- `World::get_cell`
- `World::set_cell`
- Dirty-Flags
- sichtbare Chunks

### Meilenstein 3: Rendering

- Chunk-Texturen
- Dirty-Texture-Updates
- Kamera
- Zoom
- Viewport-Culling
- Materialfarben

### Meilenstein 4: Einfache Simulation

- Sand faellt
- Wasser fliesst
- aktive Chunks
- Tick-System
- Update-Richtungswechsel

### Meilenstein 5: Spielersteuerung

- Spieler-Entity
- Input
- AABB-Kollision
- Gravity
- Springen
- Kamera folgt Spieler
- Mining
- Blockplatzierung

### Meilenstein 6: Speichern

- `serde`
- `bincode`
- `WorldHeader`
- `ChunkSaveData`
- Load/Save
- Versionierung

### Meilenstein 7: Content

- Items
- Inventar
- Gegner
- Health
- Damage
- AI

### Meilenstein 8: Licht und Debug

- Ambient Light
- Sonnenlicht
- Punktlicht
- Debug UI
- Editor Tools

## Konkreter erster Entwicklungsplan

1. Zellen, Materialien und feste Welt bauen.
2. Rendering und Mausbearbeitung umsetzen.
3. Chunks einfuehren.
4. Sand und Wasser simulieren.
5. Spieler mit Physik und Kamera bauen.
6. Speichern und Laden implementieren.
7. Items, Gegner und Licht ergaenzen.
