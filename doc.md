# Project: Blind & Scared
**Language:** C (C99 or C11)  
**Dependencies:** None (Standard Library + ANSI Escape Codes)

---

## 1. The Spatial Logic: Wave Function Collapse (WFC)
In this engine, the map does not exist. It is a "possibility space" that collapses only when the player looks at it.

### Data Structures
You will need to represent each coordinate as a set of potential tiles.
```c
typedef enum {
    TILE_VOID = 1 << 0,  // 0001
    TILE_FLOOR = 1 << 1, // 0010
    TILE_WALL = 1 << 2,  // 0100
    TILE_ALTAR = 1 << 3  // 1000
} TileType;

typedef struct {
    unsigned int entropy; // Bitmask of remaining possible TileTypes
    bool collapsed;       // Is this tile finalized?
    TileType final_type;  // The actual tile chosen
} Cell;
```

### The Adjacency Matrix
Create a "Rule Set" that defines reality. 
* *Rule:* A `TILE_ALTAR` can only be surrounded by `TILE_FLOOR`.
* *Rule:* A `TILE_VOID` cannot be adjacent to `TILE_FLOOR` without a `TILE_WALL` buffer.



---

## 2. Low-Level Terminal Manipulation
To create horror, you must control the user's screen with precision. We avoid `ncurses` to learn how terminals actually communicate with the OS.

### The Double Buffer
Instead of printing one character at a time, build the entire screen in a single `char` buffer and send it to `STDOUT` in one blast to prevent flickering.
```c
#define SCREEN_WIDTH 80
#define SCREEN_HEIGHT 24
char screen_buffer[SCREEN_WIDTH * SCREEN_HEIGHT * 20]; // Extra space for ANSI codes
```

### ANSI Escape Sequences
Use these to "glitch" the UI:
* **Move Cursor:** `\033[%d;%dH`
* **Hide Cursor:** `\033[?25l`
* **Color Shift:** `\033[38;5;%dm` (Use 256-color mode for subtle grays and deep reds)

---

## 3. The "Horror" Algorithms

### Field of Vision (FOV): Shadow Casting
The player should only see 3–5 tiles away. Use **Recursive Shadow Casting** to determine which tiles are visible. 
1. Cast rays from the player to the edge of the FOV radius.
2. If a ray hits a `TILE_WALL`, mark all tiles behind it as "Unseen."
3. **The Twist:** In this engine, tiles that are "Unseen" for too long should lose their "Collapsed" state and return to "Superposition" (erasing the map behind the player).



### The Spatial Paradox (The "Grip" of the Engine)
When the WFC algorithm tries to collapse a cell but finds **no valid tiles** (entropy == 0), the game enters a "Paradox State."
* **Implementation:** Use `setjmp` and `longjmp` in C to instantly break out of the deep recursion of the WFC and jump to a "Glitch Handler."
* **Visuals:** In the Glitch Handler, flood the `screen_buffer` with random extended ASCII characters (like `░`, `█`, `╣`) for 50ms before rewriting the map into a completely different layout.

---

## 4. The "Ghost" Memory Dump
C allows you to read your own memory. When the player dies, we want to show them the "corpse" of their session.

```c
void funeral_dump(Cell *map, int size) {
    printf("\033[2J\033[H"); // Clear screen
    printf("--- CORE SYSTEM FAILURE ---\n");
    // Print raw bytes of the map to the terminal
    unsigned char *raw = (unsigned char *)map;
    for(int i = 0; i < size * sizeof(Cell); i++) {
        printf("%02x ", raw[i]); // Print hexadecimal dump
        if(i % 16 == 0) usleep(5000); // Slow-scroll for effect
    }
}
```

---

## 5. Implementation Roadmap
1.  **Phase 1:** Write a "rule-checker" that can tell if a `Wall` is allowed next to a `Floor`.
2.  **Phase 2:** Build the WFC loop. It should fill a 20x20 grid without crashing.
3.  **Phase 3:** Implement the ANSI renderer. Get a `@` symbol moving on the screen.
4.  **Phase 4:** Add the FOV logic. The world should disappear behind you.
5.  **Phase 5:** Add "The Glitch." Force a paradox and make the terminal scream.
