# 2D-Voxel-Engine in Rust

Diese Dokumentation beschreibt eine eigene 2D-Sandbox-/Voxel-Engine in Rust.
Die Engine soll grosse zerstoerbare Welten, Zellphysik, Fluessigkeiten, Sand,
Entities, Items, Licht, Speichern/Laden und spaeter Modding unterstuetzen.

## Ziel

Das Projekt ist kein einzelnes Spiel, sondern ein technisches Fundament fuer ein
Terraria-, Falling-Sand- oder Noita-aehnliches Spiel.

## Dokumente

- [Architektur](architecture.md)
- [Welt, Chunks und Zellen](world-chunks-cells.md)
- [Materialien und Simulation](materials-simulation.md)
- [Rendering, Kamera und Licht](rendering-camera-lighting.md)
- [Entities, Physik und Gameplay](entities-physics-gameplay.md)
- [Speichern, Weltgenerierung und Modding](persistence-generation-modding.md)
- [Projektstruktur und Roadmap](project-structure-roadmap.md)

## Grundprinzip

```text
Zelle = kleinste simulierbare Einheit
Chunk = Block aus vielen Zellen
Welt = Menge aus Chunks
Entity = bewegliches Objekt ausserhalb des Zellrasters
```

Empfohlener Start:

```text
64x64 Zellen pro Chunk
Chunk-Textur-Rendering
Single-threaded Simulation
Desktop-first
Rust Workspace mit mehreren Crates
```
