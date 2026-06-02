# Rendering, Kamera und Licht

## Rendering-Ansatz

Die beste Startloesung fuer eine dynamische 2D-Zellwelt ist
Chunk-Textur-Rendering.

```text
Chunk: 64x64 Zellen
Textur: 64x64 Pixel
Jede Zelle schreibt ein Pixel in die Textur
1 Chunk = 1 Quad + 1 Textur
```

Aktuelle Empfehlung:

```text
Render: Chunk-Texturen
Licht:  eigene Light-Texturen
```

## Visual-Richtung

Solange noch kein Renderer existiert, sollten die Docs keine fake Gameplay-Szene
zeigen. Besser sind technische Diagramme fuer Pipeline, Chunk-Layout und
Datenfluss. Echte Screenshots oder GIFs kommen erst aus dem laufenden Renderer.

## Renderdaten

```rust
pub struct RenderChunk {
    pub pos: ChunkPos,
    pub texture: TextureHandle,
    pub dirty: bool,
}
```

## Pipeline

```text
World Simulation
  -> Dirty Chunks sammeln
  -> CPU-Buffer aus Cell-Daten erstellen
  -> Chunk-Textur auf GPU aktualisieren
  -> Sichtbare Chunk-Quads rendern
  -> Entity-Sprites rendern
  -> Licht-Overlay rendern
  -> UI rendern
```

## Kamera

```rust
pub struct Camera2D {
    pub position: glam::Vec2,
    pub zoom: f32,
    pub viewport_size: glam::Vec2,
}
```

## Sichtbereiche

```text
Render Radius:
  Chunks im sichtbaren Bereich

Simulation Radius:
  sichtbare Chunks + Rand

Streaming Radius:
  sichtbare Chunks + groesserer Rand
```

## Viewport-Culling

1. Bildschirmgroesse in Weltkoordinaten umrechnen.
2. Min/Max `CellPos` berechnen.
3. Daraus Min/Max `ChunkPos` berechnen.
4. Nur diese Chunks rendern.

## Lichttypen

- Ambient Light
- Sonnenlicht
- Punktlicht
- Emissive Materialien
- Schatten durch feste Zellen

## Lichtdaten

```rust
pub struct LightCell {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
```

## Sonnenlicht

Einfaches Modell:

```text
Fuer jede Spalte:
  von oben nach unten
  solange Luft/transparent: hell
  bei solidem Block: Licht nimmt ab
```

## Punktlicht

Queue-basierte Ausbreitung:

```text
Start bei Lichtquelle
Nachbarn bekommen Licht minus Daempfung
Solide Zellen blockieren oder reduzieren
```

## Rendering von Licht

Licht kann als zweite Chunk-Textur gerendert werden:

```text
Material-Textur * Licht-Textur = finale Farbe
```

## Umsetzungsschritte

1. CPU-RGBA-Buffer pro Chunk erzeugen.
2. Materialfarbe in Buffer schreiben.
3. Dirty Chunk Texturen aktualisieren.
4. Sichtbare Chunks ueber Kamera bestimmen.
5. Chunk-Quads rendern.
6. Ambient Light ergaenzen.
7. Sonnenlicht und Punktlicht hinzufuegen.
