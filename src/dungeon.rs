use crate::common::{Rect, TILE_DOOR, TILE_FLOOR, TILE_WALL};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct World {
    pub map: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Debug)]
struct Room {
    rect: Rect,
    #[allow(dead_code)]
    id: usize,
}

const MIN_DIM: usize = 5;

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        World {
            map: vec![vec![TILE_WALL; width]; height],
            width,
            height,
        }
    }

    pub fn generate(&mut self) {
        self.fill_rect(0, 0, self.width, self.height, ' ');

        let footprint = self.gen_footprint();

        let mut rooms = Vec::new();
        for rect in &footprint {
            self.recursive_slice(rect.x, rect.y, rect.w, rect.h, &mut rooms);
        }

        for r in &footprint {
            self.fill_rect(r.x, r.y, r.w, r.h, TILE_FLOOR);
        }
        for r in &footprint {
            self.draw_rect_border(r.x, r.y, r.w, r.h);
        }
        for r in &rooms {
            self.draw_rect_border(r.rect.x, r.rect.y, r.rect.w, r.rect.h);
        }

        self.connect_rooms(&rooms);
        self.place_outer_entrance(&footprint);
        self.place_hub(&rooms);
    }

    fn gen_footprint(&self) -> Vec<Rect> {
        let core_w = 10;
        let core_h = 10;
        let max_x = self.width.saturating_sub(core_w + 5).max(6);
        let max_y = self.height.saturating_sub(core_h + 5).max(6);
        let hx = 5 + rand::random::<usize>() % max_x.saturating_sub(5).max(1);
        let hy = 5 + rand::random::<usize>() % max_y.saturating_sub(5).max(1);

        let mut rects = vec![Rect {
            x: hx,
            y: hy,
            w: core_w,
            h: core_h,
        }];

        let target = 1 + rand::random::<usize>() % 3;
        let mut placed = 0;
        let mut attempts = 0;
        while placed < target && attempts < 40 {
            attempts += 1;
            let parent = rects[rand::random::<usize>() % rects.len()];
            let side = rand::random::<usize>() % 4;
            let ew = 7 + rand::random::<usize>() % 4;
            let eh = 7 + rand::random::<usize>() % 4;

            let ext = match side {
                0 => Rect {
                    x: parent.x + parent.w - 1,
                    y: parent.y + parent.h / 2 - eh / 2,
                    w: ew,
                    h: eh,
                },
                1 => {
                    if parent.x + 1 < ew {
                        continue;
                    }
                    Rect {
                        x: parent.x + 1 - ew,
                        y: parent.y + parent.h / 2 - eh / 2,
                        w: ew,
                        h: eh,
                    }
                }
                2 => Rect {
                    x: parent.x + parent.w / 2 - ew / 2,
                    y: parent.y + parent.h - 1,
                    w: ew,
                    h: eh,
                },
                _ => {
                    if parent.y + 1 < eh {
                        continue;
                    }
                    Rect {
                        x: parent.x + parent.w / 2 - ew / 2,
                        y: parent.y + 1 - eh,
                        w: ew,
                        h: eh,
                    }
                }
            };

            if ext.x == 0 || ext.y == 0 {
                continue;
            }
            if ext.x + ext.w >= self.width || ext.y + ext.h >= self.height {
                continue;
            }
            if rects.iter().any(|r| Self::overlaps_interior(r, &ext)) {
                continue;
            }

            rects.push(ext);
            placed += 1;
        }

        rects
    }

    fn recursive_slice(&self, x: usize, y: usize, w: usize, h: usize, rooms: &mut Vec<Room>) {
        let can_v = w >= MIN_DIM * 2 - 1;
        let can_h = h >= MIN_DIM * 2 - 1;

        if can_v && (rand::random::<bool>() || !can_h) {
            let span = w + 1 - 2 * MIN_DIM;
            let split = MIN_DIM + rand::random::<usize>() % (span + 1);
            self.recursive_slice(x, y, split, h, rooms);
            self.recursive_slice(x + split - 1, y, w - split + 1, h, rooms);
        } else if can_h {
            let span = h + 1 - 2 * MIN_DIM;
            let split = MIN_DIM + rand::random::<usize>() % (span + 1);
            self.recursive_slice(x, y, w, split, rooms);
            self.recursive_slice(x, y + split - 1, w, h - split + 1, rooms);
        } else {
            rooms.push(Room {
                rect: Rect { x, y, w, h },
                id: rooms.len(),
            });
        }
    }

    fn connect_rooms(&mut self, rooms: &[Room]) {
        const MIN_DOOR_SPACING: usize = 3;

        let mut adj: HashMap<usize, Vec<(usize, (usize, usize, bool))>> = HashMap::new();
        for i in 0..rooms.len() {
            for j in i + 1..rooms.len() {
                if let Some(pos) = self.find_door_pos(&rooms[i].rect, &rooms[j].rect) {
                    adj.entry(i).or_default().push((j, pos));
                    adj.entry(j).or_default().push((i, pos));
                }
            }
        }

        // placed doors keyed by wall line: (axis, position) -> sorted list of offsets
        // axis true = vertical wall (col fixed), false = horizontal wall (row fixed)
        let mut placed: HashMap<(bool, usize), Vec<usize>> = HashMap::new();

        let too_close = |placed: &HashMap<(bool, usize), Vec<usize>>,
                         axis: bool,
                         pos: usize,
                         off: usize|
         -> bool {
            placed
                .get(&(axis, pos))
                .map(|v| v.iter().any(|&o| o.abs_diff(off) < MIN_DOOR_SPACING))
                .unwrap_or(false)
        };

        let mut visited = HashSet::new();
        for start in 0..rooms.len() {
            if visited.contains(&start) {
                continue;
            }
            let mut q = VecDeque::new();
            q.push_back(start);
            visited.insert(start);
            while let Some(u) = q.pop_front() {
                if let Some(neighbors) = adj.get(&u) {
                    let mut ns = neighbors.clone();
                    ns.sort_by_key(|&(v, _)| v);
                    for (v, (px, py, vert)) in ns {
                        if visited.contains(&v) {
                            continue;
                        }
                        let pos = if vert { px } else { py };
                        let off = if vert { py } else { px };
                        if too_close(&placed, vert, pos, off) {
                            // try alternate position on same wall
                            if let Some(alt) = self.find_door_pos_avoiding(
                                &rooms[u].rect,
                                &rooms[v].rect,
                                &placed,
                                MIN_DOOR_SPACING,
                            ) {
                                let (apx, apy, avert) = alt;
                                if self.map[apy][apx] == TILE_WALL {
                                    self.map[apy][apx] = TILE_DOOR;
                                }
                                let apos = if avert { apx } else { apy };
                                let aoff = if avert { apy } else { apx };
                                placed.entry((avert, apos)).or_default().push(aoff);
                                visited.insert(v);
                                q.push_back(v);
                                continue;
                            }
                        }
                        if self.map[py][px] == TILE_WALL {
                            self.map[py][px] = TILE_DOOR;
                        }
                        placed.entry((vert, pos)).or_default().push(off);
                        visited.insert(v);
                        q.push_back(v);
                    }
                }
            }
        }
    }

    fn find_door_pos(&self, a: &Rect, b: &Rect) -> Option<(usize, usize, bool)> {
        let vshared = if a.x + a.w - 1 == b.x {
            Some(a.x + a.w - 1)
        } else if b.x + b.w - 1 == a.x {
            Some(a.x)
        } else {
            None
        };
        if let Some(col) = vshared {
            let y0 = a.y.max(b.y) + 1;
            let y1 = (a.y + a.h).min(b.y + b.h).saturating_sub(1);
            if y1 > y0 && col > 0 && col + 1 < self.width {
                let cands: Vec<usize> = (y0..y1)
                    .filter(|&y| {
                        self.map[y][col - 1] == TILE_FLOOR
                            && self.map[y][col + 1] == TILE_FLOOR
                    })
                    .collect();
                if !cands.is_empty() {
                    let pick = cands[cands.len() / 2];
                    return Some((col, pick, true));
                }
            }
        }

        let hshared = if a.y + a.h - 1 == b.y {
            Some(a.y + a.h - 1)
        } else if b.y + b.h - 1 == a.y {
            Some(a.y)
        } else {
            None
        };
        if let Some(row) = hshared {
            let x0 = a.x.max(b.x) + 1;
            let x1 = (a.x + a.w).min(b.x + b.w).saturating_sub(1);
            if x1 > x0 && row > 0 && row + 1 < self.height {
                let cands: Vec<usize> = (x0..x1)
                    .filter(|&x| {
                        self.map[row - 1][x] == TILE_FLOOR
                            && self.map[row + 1][x] == TILE_FLOOR
                    })
                    .collect();
                if !cands.is_empty() {
                    let pick = cands[cands.len() / 2];
                    return Some((pick, row, false));
                }
            }
        }

        None
    }

    fn find_door_pos_avoiding(
        &self,
        a: &Rect,
        b: &Rect,
        placed: &HashMap<(bool, usize), Vec<usize>>,
        spacing: usize,
    ) -> Option<(usize, usize, bool)> {
        let far_enough = |axis: bool, pos: usize, off: usize| -> bool {
            placed
                .get(&(axis, pos))
                .map(|v| v.iter().all(|&o| o.abs_diff(off) >= spacing))
                .unwrap_or(true)
        };

        let vshared = if a.x + a.w - 1 == b.x {
            Some(a.x + a.w - 1)
        } else if b.x + b.w - 1 == a.x {
            Some(a.x)
        } else {
            None
        };
        if let Some(col) = vshared {
            let y0 = a.y.max(b.y) + 1;
            let y1 = (a.y + a.h).min(b.y + b.h).saturating_sub(1);
            if y1 > y0 && col > 0 && col + 1 < self.width {
                let cands: Vec<usize> = (y0..y1)
                    .filter(|&y| {
                        self.map[y][col - 1] == TILE_FLOOR
                            && self.map[y][col + 1] == TILE_FLOOR
                            && far_enough(true, col, y)
                    })
                    .collect();
                if !cands.is_empty() {
                    return Some((col, cands[cands.len() / 2], true));
                }
            }
        }

        let hshared = if a.y + a.h - 1 == b.y {
            Some(a.y + a.h - 1)
        } else if b.y + b.h - 1 == a.y {
            Some(a.y)
        } else {
            None
        };
        if let Some(row) = hshared {
            let x0 = a.x.max(b.x) + 1;
            let x1 = (a.x + a.w).min(b.x + b.w).saturating_sub(1);
            if x1 > x0 && row > 0 && row + 1 < self.height {
                let cands: Vec<usize> = (x0..x1)
                    .filter(|&x| {
                        self.map[row - 1][x] == TILE_FLOOR
                            && self.map[row + 1][x] == TILE_FLOOR
                            && far_enough(false, row, x)
                    })
                    .collect();
                if !cands.is_empty() {
                    return Some((cands[cands.len() / 2], row, false));
                }
            }
        }

        None
    }

    fn place_outer_entrance(&mut self, footprint: &[Rect]) {
        let inside = |x: usize, y: usize| -> bool {
            footprint
                .iter()
                .any(|r| x >= r.x && x < r.x + r.w && y >= r.y && y < r.y + r.h)
        };

        let mut cands: Vec<(usize, usize)> = Vec::new();
        for r in footprint {
            for x in r.x + 1..r.x + r.w - 1 {
                if r.y > 0
                    && self.map[r.y][x] == TILE_WALL
                    && !inside(x, r.y - 1)
                    && self.map[r.y + 1][x] == TILE_FLOOR
                {
                    cands.push((x, r.y));
                }
                let by = r.y + r.h - 1;
                if by + 1 < self.height
                    && self.map[by][x] == TILE_WALL
                    && !inside(x, by + 1)
                    && self.map[by - 1][x] == TILE_FLOOR
                {
                    cands.push((x, by));
                }
            }
            for y in r.y + 1..r.y + r.h - 1 {
                if r.x > 0
                    && self.map[y][r.x] == TILE_WALL
                    && !inside(r.x - 1, y)
                    && self.map[y][r.x + 1] == TILE_FLOOR
                {
                    cands.push((r.x, y));
                }
                let rx = r.x + r.w - 1;
                if rx + 1 < self.width
                    && self.map[y][rx] == TILE_WALL
                    && !inside(rx + 1, y)
                    && self.map[y][rx - 1] == TILE_FLOOR
                {
                    cands.push((rx, y));
                }
            }
        }
        if cands.is_empty() {
            return;
        }
        let (x, y) = cands[rand::random::<usize>() % cands.len()];
        self.map[y][x] = TILE_DOOR;
    }

    fn place_hub(&mut self, rooms: &[Room]) {
        if let Some(hub) = rooms.iter().max_by_key(|r| r.rect.w * r.rect.h) {
            let cx = hub.rect.x + hub.rect.w / 2;
            let cy = hub.rect.y + hub.rect.h / 2;
            self.safe_set_tile(cx, cy, 'K');
        }
    }

    fn overlaps_interior(a: &Rect, b: &Rect) -> bool {
        let ix_lo = a.x.max(b.x);
        let ix_hi = (a.x + a.w).min(b.x + b.w);
        let iy_lo = a.y.max(b.y);
        let iy_hi = (a.y + a.h).min(b.y + b.h);
        ix_hi > ix_lo + 1 && iy_hi > iy_lo + 1
    }

    fn draw_rect_border(&mut self, x: usize, y: usize, w: usize, h: usize) {
        (x..x + w).for_each(|col| {
            self.safe_set_tile(col, y, TILE_WALL);
            self.safe_set_tile(col, y + h - 1, TILE_WALL);
        });
        (y..y + h).for_each(|row| {
            self.safe_set_tile(x, row, TILE_WALL);
            self.safe_set_tile(x + w - 1, row, TILE_WALL);
        });
    }

    fn safe_set_tile(&mut self, x: usize, y: usize, tile: char) {
        if y < self.height && x < self.width {
            self.map[y][x] = tile;
        }
    }

    fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, tile: char) {
        let width = self.width;
        let height = self.height;

        (y..y + h)
            .filter(|&row| row < height)
            .flat_map(|row| (x..x + w).map(move |col| (row, col)))
            .filter(|&(_, col)| col < width)
            .for_each(|(row, col)| {
                self.map[row][col] = tile;
            });
    }
}
