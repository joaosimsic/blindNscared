# BLIND N' SCARED
### Project Core Description

---

## 1. CONCEPT OVERVIEW
**Blind N' Scared** is a terminal-based, asymmetrical horror game developed in **Rust**. It pits a group of Survivors against a single Monster in a procedurally generated environment. The experience emphasizes atmospheric dread through restricted visibility, directional sound, and a centered POV.

---

## 2. THE VISION ENGINE
The rendering pipeline is a three-tier system calculated per-frame relative to the player's `@` coordinates.

### 2.1 Line of Sight (LOS) Calculation
Visibility is determined using **Recursive Shadowcasting**. 
* **Origin:** The player's tile `@`.
* **Blocking Tiles:** Walls `█` and Closed Doors `/` terminate the visibility ray.
* **Result:** A set of "Visible Coordinates" is generated every time the player moves or an object in the world changes state.

### 2.2 Visibility Tiers
1.  **Tier 1: Clear Zone (Radius: $r_1$):** All tiles within LOS and $r_1$ distance are rendered using the [Tileset Definition](#4-tileset-definition).
2.  **Tier 2: Detection Range (Radius: $r_2$):** Tiles within LOS but beyond $r_1$ are rendered as `The Void` (Black), **unless** they contain a dynamic entity (Survivor, Monster, Exit). Those entities are rendered as `?`.
3.  **Tier 3: The Void:** Any tile outside LOS or beyond $r_2$ is strictly not rendered (Black).

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

---

## 7. MULTIPLAYER ARCHITECTURE
**Blind N' Scared** uses a **Listen Server** model. One player acts as both a Client and the Host, while others connect as Clients via UDP.

### Host-Client Responsibilities
| Role | Responsibility |
| :--- | :--- |
| **Host (Server)** | Validates movement, runs Monster AI, manages map state, and calculates LOS for all players. |
| **Client** | Captures input, performs local prediction, and renders the world based on server packets. |

### Networking Strategy
* **Protocol:** UDP (via `quinn` or `renet`) for low-latency movement updates.
* **Client-Side Prediction:** To eliminate input lag, the local player `@` moves instantly on the client's screen. If the Host rejects the move (e.g., player hit a wall), the client "rubber-bands" back to the valid position.
* **Information Culling (Anti-Cheat):** The Host only sends the coordinates of other players/monsters if they are within the recipient's **Detection Range** (`?`) or **Clear Zone**. The client's memory never contains the full map state.

---

## 8. REAL-TIME ENGINE & SYNC
The game operates on a dual-update loop to balance real-time fluidness with server authority.

### The Tick System
* **Global Tick:** Every 50ms (20Hz), the Host broadcasts a "World State" packet containing moved entities and state changes (e.g., opened doors).
* **Inter-Tick Interpolation:** Clients can smoothly transition the `?` markers between tick positions to prevent "stuttery" movement in the terminal.

### Action Propagation
1. **Input:** Player presses `W`.
2. **Local Render:** `@` moves North immediately.
3. **Packet:** A `MoveRequest` is sent to the Host.
4. **Validation:** Host confirms the move and updates the global map.
5. **Broadcast:** Host sends an `EntityMoved` packet to all players who can "see" or "hear" that movement.

---

## 9. LOBBY & CONNECTION
* **Direct Connect:** Clients connect via `IPv4:Port`.
* **Discovery:** Integration of **UPnP** to automatically handle port forwarding for the Host, allowing external friends to join without manual router configuration.
* **Syncing Seeds:** The procedural map is not sent as a large file. Instead, the Host sends the **Seed (u64)** during the handshake, and all clients generate the identical map locally.

---

### Suggested Technical Stack for Rust
To implement this, you might add these to your technical considerations:
* **`serde` + `bincode`:** For ultra-fast, small binary serialization of packets.
* **`tokio` or `mio`:** For handling asynchronous network I/O without blocking the render thread.
* **`crossbeam-channel`:** To pass data between your network thread and your game logic thread.

---

## 10. RENDERING & PIPELINE LOGIC
Because this is a centered-POV game, the renderer must translate **Global Map Coordinates** to **Local Terminal Coordinates**.

### 10.1 Coordinate Transformation
The terminal center is always `(TermWidth/2, TermHeight/2)`. 
For any entity at `(map_x, map_y)`, its screen position is:
$$Screen_X = (map\_x - player\_x) + (TermWidth / 2)$$
$$Screen_Y = (map\_y - player\_y) + (TermHeight / 2)$$

### 10.2 The Render Pass
On every tick or input event, the engine performs these steps:
1.  **Clear Buffer:** Wipe the previous frame.
2.  **Compute LOS:** Run the shadowcasting algorithm from the player's current position.
3.  **Iterate Viewport:** For every terminal cell `(tx, ty)`:
    * Map the cell to a global coordinate `(gx, gy)`.
    * **IF** `(gx, gy)` is not in LOS $\rightarrow$ Render ` ` (Black).
    * **ELSE IF** distance to `(gx, gy)` $\le$ Clear Zone $\rightarrow$ Render actual tile from map.
    * **ELSE IF** distance $\le$ Detection Range **AND** tile contains Entity $\rightarrow$ Render `?`.
    * **ELSE** $\rightarrow$ Render ` ` (Black).
4.  **Draw UI Overlay:** Render the text log and status bars over the world view.

---

### 10.3 Dynamic Occlusion (Real-Time)
Since the game is real-time, the LOS must account for moving occluders. If the Monster `M` moves behind a wall `█`, the `?` indicator must vanish **instantly**. 
* **Host Authority:** In multiplayer, the Host calculates the LOS for each client and only sends the data the client is allowed to see.
* **Flicker Effect:** To simulate a failing flashlight or high stress, the radii ($r_1, r_2$) can be programmatically jittered during the Render Pass.

---

## 11. GAME FLOW & STATE MACHINE 

### 11.1 The Setup Phase
Before the map is baked, the Host (or Solo Player) defines the "Rules of Engagement."
* **Map Constraints:** Width ($W$) and Height ($H$).
* **Entity Count:** * **Survivors:** $1$ to $N$ (Players or Bots).
    * **Monster:** $1$ (Player or Bot).
* **Spawn Buffer ($d$):** A variable that ensures no Survivor spawns within $d$ tiles of the Monster.

### 11.2 Phase 1: The "Cold" Spawn
The game initialization follows a strict geometric check to ensure fairness while maintaining randomness:
1.  **Monster First:** The Monster is placed at a random valid floor tile $(M_x, M_y)$.
2.  **Survivor Scatter:** For each Survivor, the engine picks a random coordinate $(S_x, S_y)$.
    * **Validation:** If $Distance(M, S) < d$, the coordinate is rerolled.
    * **Result:** Survivors start scattered across the map, potentially in total darkness and completely alone.
3.  **Exit Placement:** Two `X` tiles are placed in opposite quadrants of the map.



---

## 12. DYNAMIC GAMEPLAY MODES

Since you want the ability to toggle AI and player counts, the game flow adapts based on the "Slot" configuration:

### 12.1 Single Player Flow
* **Player:** Assigned to either Survivor or Monster.
* **AI Fill:** The engine automatically populates the opposing side with the requested number of bots.
* **Pause Capability:** Since the server is local, the "Global Tick" can be suspended.

### 12.2 Multiplayer Flow
* **Lobby:** Players "Claim" a slot (Survivor or Monster). Unclaimed slots are filled by AI if the host enabled them.
* **Persistence:** If a player disconnects, an AI "Brain" can take over their `@` character so the match doesn't break.

---

## 13. VICTORY & DEFEAT PATHS

The game flow concludes when one of these terminal states is reached:

| Condition | Outcome | Narrative Feedback |
| :--- | :--- | :--- |
| **Survivor enters `X`** | **Survivor Victory** | "You escaped the void." (Only the escaped player wins). |
| **All Survivors Dead** | **Monster Victory** | "None remained to tell the tale." |
| **All Exits Blocked?** | **Stalemate** | *Technical Guard:* The procedural gen must ensure paths to `X` are never fully walled off. |

---

## 14. TECHNICAL IMPLEMENTATION: THE "HEARTBEAT" LOOP
Because you are using Rust, you can handle the game flow efficiently using an **Enum-based State Machine**.

```rust
enum GameState {
    Lobby,
    Generating,
    Spawning, // This is where your random distance check happens
    ActivePlay,
    GameOver { winner: Side }
}
```

### The "Tick" Logic
1.  **Process Input:** Collect movement from local/network/AI.
2.  **Resolve Collision:** Ensure nobody walks through walls `█`.
3.  **Update LOS:** Calculate what every player sees based on the Tier system.
4.  **Check Win Condition:** Did a Survivor coordinate match an `X` coordinate?
5.  **Broadcast:** Send the minimal update packet to all clients.

---

### Procedural Tip: The "Minimal Distance" Algorithm
To avoid infinite loops during spawning in small maps, use a **Poisson Disk Sampling** approach or a simple "Attempt Counter." If the engine fails to find a spot for a survivor after 100 tries, it slightly reduces the distance $d$ until a spot is found.


