# Materialien und Simulation

## Materialien

Materialien beschreiben Verhalten, Aussehen und physikalische Eigenschaften von
Zellen.

```rust
pub enum MaterialKind {
    Empty,
    Solid,
    Powder,
    Liquid,
    Gas,
}
```

```rust
pub struct MaterialDef {
    pub id: MaterialId,
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
```

## Material-Registry

```rust
pub struct MaterialRegistry {
    materials: Vec<MaterialDef>,
}

impl MaterialRegistry {
    pub fn get(&self, id: MaterialId) -> &MaterialDef {
        &self.materials[id.0 as usize]
    }
}
```

Fuer Modding sollten Materialien spaeter externe String-IDs erhalten, zum
Beispiel `core:stone` oder `my_mod:blue_sand`.

## Simulationsprinzip

Pro Tick werden aktive Zellen untersucht.

Die Simulation sollte chunk-basiert organisiert werden. Chunks koennen dadurch
aktiviert, deaktiviert, als dirty markiert und spaeter gezielt parallelisiert
werden.

```text
Sand:
  nach unten fallen
  sonst diagonal rutschen

Wasser:
  nach unten fliessen
  sonst seitlich verteilen

Lava:
  wie Wasser, aber langsamer
  reagiert mit Wasser
```

## Update-Reihenfolge

Falling-Sand-Simulationen sind stark von der Reihenfolge abhaengig.

Empfehlung:

```text
Von unten nach oben iterieren.
Horizontalrichtung pro Tick wechseln.

Tick gerade: links nach rechts
Tick ungerade: rechts nach links
```

## Sand

```rust
fn simulate_sand(world: &mut World, pos: CellPos) {
    let below = CellPos { x: pos.x, y: pos.y - 1 };
    let down_left = CellPos { x: pos.x - 1, y: pos.y - 1 };
    let down_right = CellPos { x: pos.x + 1, y: pos.y - 1 };

    if world.is_empty(below) {
        world.swap_cells(pos, below);
    } else if world.is_empty(down_left) {
        world.swap_cells(pos, down_left);
    } else if world.is_empty(down_right) {
        world.swap_cells(pos, down_right);
    }
}
```

Ob `y - 1` nach unten bedeutet, haengt vom Koordinatensystem ab. Wichtig ist
nur Konsistenz.

## Wasser

Ein einfaches Modell:

```text
Jede Wasserzelle hat amount 0..255.
Wasser fliesst zuerst nach unten.
Wenn unten voll oder blockiert ist, verteilt es sich horizontal.
```

Regeln:

```text
Wenn unten leer:
  bewege moeglichst viel Wasser nach unten.

Wenn unten teilweise gefuellt:
  fuelle unten auf.

Wenn unten blockiert:
  verteile nach links/rechts.
```

## Lava

Lava nutzt aehnliche Regeln wie Wasser, aber:

- niedrigere Update-Frequenz
- hoehere Viskositaet
- Emissive-Licht
- Schaden an Entities
- Reaktion mit Wasser

## Active-Chunk-System

Ein Chunk ist aktiv, wenn:

- sich darin kuerzlich etwas bewegt hat
- Fluessigkeit oder Sand vorhanden ist
- Spieler in der Naehe ist
- ein Nachbar-Chunk aktiv ist
- eine Explosion oder Blockaenderung passiert ist

```rust
pub struct SimulationState {
    pub active_chunks: Vec<ChunkPos>,
}
```

## Risiken

- Wasser kann oszillieren.
- Zellen koennen mehrfach pro Tick bewegt werden.
- Chunk-Grenzen erzeugen Bugs.
- Multithreading ist bei direkter Mutation schwierig.

## Umsetzungsschritte

1. Sand implementieren.
2. `UPDATED_THIS_TICK` einfuehren.
3. Wasser ohne Drucksystem implementieren.
4. Dirty-Simulation-Chunks markieren.
5. Lava mit niedriger Update-Frequenz.
6. Materialreaktionen hinzufuegen.
