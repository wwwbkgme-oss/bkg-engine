# 2D-Voxel-Engine in Rust

Diese Dokumentation beschreibt eine eigene 2D-Sandbox-/Voxel-Engine in Rust.
Die Engine soll grosse zerstoerbare Welten, Zellphysik, Fluessigkeiten, Sand,
Entities, Items, Licht, Speichern/Laden und spaeter Modding unterstuetzen.

## Ziel

Das Projekt ist kein einzelnes Spiel, sondern ein technisches Fundament fuer ein
Terraria-, Falling-Sand- oder Noita-aehnliches Spiel.

Die Doku verkauft deshalb keine fertige Gameplay-Szene. Sie zeigt technische
Architektur, Datenfluss, Entscheidungen und Roadmap. Echte Screenshots oder GIFs
sollten erst verwendet werden, wenn ein Renderer im Repository existiert.

## Dokumente

- [Architektur](architecture.md)
- [Welt, Chunks und Zellen](world-chunks-cells.md)
- [Materialien und Simulation](materials-simulation.md)
- [Rendering, Kamera und Licht](rendering-camera-lighting.md)
- [Entities, Physik und Gameplay](entities-physics-gameplay.md)
- [Speichern, Weltgenerierung und Modding](persistence-generation-modding.md)
- [Projektstruktur und Roadmap](project-structure-roadmap.md)
- [Visual Guidelines](visual-guidelines.md)

## Grundprinzip

```text
Zelle = kleinste simulierbare Einheit
Chunk = Block aus vielen Zellen
Welt = Menge aus Chunks
Entity = bewegliches Objekt ausserhalb des Zellrasters
```

## Aktuelle technische Empfehlung

```text
Chunk:      64x64
Region:     2x2 Chunks
Render:     Chunk-Texturen
Licht:      eigene Light-Texturen
Simulation: Chunk-basiert
```

Das ist ein guter Kompromiss aus Qualitaet, Skalierbarkeit und Performance.
