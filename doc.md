# BLIND N' SCARED
### Project Core Description

---

## 1. CONCEPT OVERVIEW
**Blind N' Scared** is a terminal-based, asymmetrical horror game developed in **Rust**. It pits a group of Survivors against a single Monster in a procedurally generated environment. The experience emphasizes atmospheric dread through restricted visibility, directional sound, and a centered POV.

---

## 2. THE VISION ENGINE
The game’s primary mechanic is the limitation of player information.

### Point of View (POV)
The terminal camera is strictly centered on the player character at all times. As the player moves, the world shifts around them.

### Visibility & The Detection Marker
* **Clear Zone:** A small radius around the player where all tiles and entities are correctly identified and rendered.
* **The Void:** Any floor or wall tile outside the Clear Zone is not rendered. The player sees only blackness.
* **The "?" Indicator:** If a **Survivor**, **Monster**, or **Exit** is located outside the Clear Zone but within a certain detection range, it is rendered as a `?`.
    * This forces the player to decide whether to approach a mystery mark that could be their escape, an ally, or their death.
* **Occlusion:** Walls and closed doors block all vision and the `?` indicator. You cannot "detect" through solid objects.

---

## 3. PROCEDURAL WORLD GENERATION
Each match uses a unique seed to generate a layout consisting of:
* **Houses & Rooms:** Enclosed spaces designed for hiding and navigation.
* **Corridors:** Open pathways that connect different structures.
* **Dual Exits:** Every map generates exactly two Exit points. This ensures the Monster cannot effectively block the escape by sitting on a single tile.

---

## 4. TILESET DEFINITION
The game uses a mix of standard ASCII and specific Unicode block characters for a gritty, technical look.

| Character | Type | Property |
| :--- | :--- | :--- |
| ` ` | Floor | Walkable, allows LOS. |
| `█` | Wall | Impassable, blocks LOS. |
| `/` | Door | Toggleable; blocks LOS when closed. |
| `X` | Exit | Survival win condition (2 per map). |
| `?` | Unknown | Placeholder for any tile in the Unclear Zone. |

---

## 5. PLAYER REPRESENTATION
* **@**: The Local Player.
* **S**: A Survivor (visible only in Clear Zone).
* **M**: The Monster (visible only in Clear Zone).

---

## 6. CORE LOGIC & SOUND
* **Movement:** Grid-based movement (WASD/Arrows).
* **Audio Cues:** Since visibility is low, players receive text-based or minimal audio feedback regarding proximity (e.g., "You hear a door open to the North").
* **Rust Implementation:** High-performance handling of the FOV/LOS algorithm and procedural seed generation.
