# Architektur

## Kernidee

Die Engine simuliert eine grosse 2D-Welt aus diskreten Zellen. Jede Zelle
enthaelt ein Material wie Luft, Erde, Stein, Sand, Wasser oder Lava. Bewegliche
Objekte wie Spieler, Gegner, Items und Projektile werden als Entities ausserhalb
des Zellrasters verwaltet.

## Schichten

```text
Game Layer
  Regeln, Items, Gegner, Gameplay

Engine Core
  World, Chunks, Cells
  Materials
  Simulation
  Physics
  Entities
  Items
  Generation
  Lighting
  Save/Load

Renderer
  Chunk-Rendering
  Entity-Rendering
  Lighting-Overlay
  Debug-Rendering

App Layer
  Window
  Input
  Game Loop
  Platform
```

## Zielplattformen

Primaer:

- Windows
- Linux
- macOS

Spaeter optional:

- WebAssembly
- Android
- iOS

Desktop-first ist sinnvoll, weil Debugging, Dateisystem, Multithreading und
GPU-APIs dort einfacher zu handhaben sind.

## Trennung der Verantwortlichkeiten

```text
engine_core kennt kein winit, kein wgpu und kein Betriebssystem.
engine_app kuemmert sich um Fenster, Input und Plattform.
engine_render kuemmert sich um GPU und Darstellung.
sandbox_game registriert konkrete Inhalte und Gameplay-Regeln.
```

## Empfohlene Libraries

```toml
wgpu = "0.x"
winit = "0.x"
glam = "0.x"
serde = { version = "1", features = ["derive"] }
bincode = "1"
rand = "0.8"
noise = "0.x"
```

Spaeter:

```toml
rayon = "1"
crossbeam-channel = "0.x"
egui = "0.x"
lz4_flex = "0.x"
zstd = "0.x"
```

## Frame-Ablauf

```text
Input sammeln
  -> Player Input System
  -> AI System
  -> Physics System
  -> World Simulation
  -> Lighting Update
  -> Streaming / Save Queue
  -> Render Extraction
  -> GPU Update
  -> Draw
```

## Wichtige Architekturentscheidungen

- Chunk-Groesse: `64x64` Zellen.
- Rendering: Chunk-Texturen statt einzelne Zell-Quads.
- Simulation: zuerst single-threaded korrekt machen.
- Materialverhalten: Materialdaten plus Simulationssysteme.
- ECS: einfach starten, spaeter ausbauen.
- Savegames: von Anfang an versionieren.
