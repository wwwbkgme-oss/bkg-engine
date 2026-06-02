# Speichern, Weltgenerierung und Modding

## Speichern und Laden

Welten sollten nicht komplett als ein riesiger Block gespeichert werden. Besser
ist chunk- oder regionbasiertes Speichern.

Einfache erste Version:

```text
world.bin
```

Bessere spaetere Version:

```text
saves/my_world/
  world.meta
  regions/
    r.0.0.region
    r.0.1.region
    r.-1.0.region
```

## Save Header

```rust
#[derive(Serialize, Deserialize)]
pub struct WorldHeader {
    pub version: u32,
    pub seed: u64,
    pub tick: u64,
    pub generator_id: String,
}
```

## Chunk Save Data

```rust
#[derive(Serialize, Deserialize)]
pub struct ChunkSaveData {
    pub pos: ChunkPos,
    pub cells: Vec<Cell>,
}
```

## Kompression

Viele Chunks bestehen aus langen Bereichen gleicher Materialien.

Sinnvoll:

```text
RLE fuer gleiche Zellen
danach lz4 oder zstd
```

## Weltgenerierung

Chunks sollten deterministisch aus einem Seed generiert werden.

```rust
pub trait WorldGenerator {
    fn generate_chunk(&self, pos: ChunkPos, chunk: &mut Chunk);
}
```

Generator-Schichten:

1. Hoehenprofil
2. Oberflaeche
3. Erde
4. Stein
5. Hoehlen
6. Erze
7. Wasserbecken
8. Biome
9. Strukturen

## Beispielstrategie

```text
surface_y = noise(x) * amplitude + base_height

Wenn y > surface_y:
  Luft

Wenn y == surface_y:
  Gras/Erde

Wenn y < surface_y:
  Erde oder Stein

Wenn cave_noise(x, y) > threshold:
  Luft
```

## Modding

Zuerst Daten-Modding, kein nativer Code.

```text
mods/
  example_mod/
    mod.toml
    materials.ron
    items.ron
    recipes.ron
    biomes.ron
```

## Manifest

```rust
pub struct ModManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
}
```

## Externe IDs

Fuer Mods niemals nur numerische IDs verwenden.

```text
core:stone
core:water
my_mod:blue_sand
```

Intern kann daraus eine `MaterialId(u16)` werden.

## Umsetzungsschritte

1. Einfache vollstaendige Welt speichern.
2. `serde` und `bincode` verwenden.
3. Header mit Version einfuehren.
4. Chunkweise Speicherung bauen.
5. Kompression ergaenzen.
6. Flache Testwelt generieren.
7. Heightmap und Hoehlennoise hinzufuegen.
8. Registries mit String-IDs bauen.
9. Materialien und Items aus Dateien laden.
