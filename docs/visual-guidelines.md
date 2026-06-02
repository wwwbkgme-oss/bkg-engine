# Visual Guidelines

## Ziel

Die Visuals sollen die Engine erklaeren, nicht ein fertiges Spiel vortaeuschen.
Solange es keinen Renderer gibt, werden keine fake Gameplay-Szenen, keine
handgemalten Welt-Screenshots und keine erfundenen UI-Screenshots verwendet.

## Stilrichtung

Die Optik darf modern, dunkel und technisch wirken:

```text
ruhige Engine-UI
klare Panels
feine Linien
neon-gelbe Akzente
cyan/pink nur sparsam fuer Signale
monospace Labels fuer technische Werte
```

Der Stil kann cyberpunk-inspiriert sein, aber die Inhalte bleiben sachlich:
Architektur, Datenfluss, Chunk-Layout, Render-Pipeline, Save/Load und Debugging.

## Erlaubte Visuals

- Chunk/Data-Flow-Diagramme
- Layer- und Crate-Diagramme
- Render-Pipeline-Diagramme
- Speicherlayout fuer Cells, Chunks und Regions
- echte Screenshots oder GIFs aus dem laufenden Renderer
- Debug-Overlays, wenn sie wirklich aus dem Projekt stammen

## Nicht verwenden

- fake Gameplay-Szenen
- dekorative Pixel-Art-Welten ohne Bezug zum aktuellen Code
- Screenshots, die Features zeigen, die noch nicht existieren
- Marketing-Hero-Grafiken, die wie ein fertiges Spiel wirken

## Konkrete Richtung

Das Repository sollte eher wie ein technisches Engine-Dashboard wirken:

```text
Cell -> Chunk -> World -> Simulation -> Dirty Chunks -> Renderer
```

Diese Kette ist wichtiger als eine atmosphaerische Spielszene.
