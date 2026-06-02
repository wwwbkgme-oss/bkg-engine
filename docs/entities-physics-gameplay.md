# Entities, Physik und Gameplay

## Entities

Entities sind bewegliche Objekte ausserhalb des Zellrasters:

- Spieler
- Gegner
- Items
- Projektile
- Partikel
- NPCs

## Spieler

```rust
pub struct Player;

pub struct Transform {
    pub position: glam::Vec2,
}

pub struct Velocity {
    pub value: glam::Vec2,
}

pub struct Collider {
    pub size: glam::Vec2,
}

pub struct Health {
    pub current: i32,
    pub max: i32,
}
```

## Input

```rust
pub struct InputState {
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub mine: bool,
    pub place: bool,
    pub selected_slot: usize,
    pub mouse_world_pos: glam::Vec2,
}
```

## Physik

Fuer Spieler und Gegner reicht am Anfang AABB-Kollision.

```rust
pub struct Aabb {
    pub min: glam::Vec2,
    pub max: glam::Vec2,
}

pub struct PhysicsBody {
    pub velocity: glam::Vec2,
    pub gravity_scale: f32,
    pub grounded: bool,
}
```

## Kollisionsablauf

```text
1. Velocity durch Gravity aendern.
2. X-Bewegung testen.
3. Kollision auf X loesen.
4. Y-Bewegung testen.
5. Kollision auf Y loesen.
6. grounded setzen.
```

## Items

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(pub u16);

pub struct ItemDef {
    pub id: ItemId,
    pub name: String,
    pub max_stack: u16,
    pub kind: ItemKind,
}

pub enum ItemKind {
    Block(MaterialId),
    Tool(ToolDef),
    Weapon(WeaponDef),
    Consumable,
    Resource,
}

pub struct ItemStack {
    pub item: ItemId,
    pub amount: u16,
}
```

## Gameplay

```text
Block abbauen -> Item erzeugen
Item aufsammeln -> Inventar
Block-Item benutzen -> Zelle setzen
Werkzeug benutzen -> schneller abbauen
```

## Gegner und NPCs

```rust
pub enum AiMode {
    Idle,
    Wander,
    Chase(EntityId),
    Attack(EntityId),
    Flee,
}
```

Fuer fruehe Versionen reicht einfache Bodenbewegung:

- Idle
- Wander
- Chase Player
- Attack
- kleine Hindernisse ueberspringen
- bei Abgrund umdrehen

## ECS-Empfehlung

Nicht sofort ein grosses ECS bauen. Erst einfach starten:

```text
Player struct
Vec<Enemy>
Vec<ItemDrop>
```

Spaeter kann daraus ein SparseSet-ECS werden.

## Umsetzungsschritte

1. Spieler-Entity erstellen.
2. Input-System bauen.
3. Gravity und AABB-Kollision implementieren.
4. Kamera dem Spieler folgen lassen.
5. Mining und Platzierung ergaenzen.
6. Item-Drops und Pickup-System hinzufuegen.
7. Gegner mit einfacher AI bauen.
