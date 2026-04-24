use crate::common::{Rect, TILE_DOOR, TILE_FLOOR, TILE_WALL};
use std::collections::{HashSet, VecDeque};

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
    #[allow(dead_code)]
    is_hallway: bool,
}

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

        let hx = 5 + rand::random::<usize>() % (self.width.saturating_sub(30).max(1));
        let hy = 5 + rand::random::<usize>() % (self.height.saturating_sub(30).max(1));

        let mut house_rects = vec![Rect {
            x: hx,
            y: hy,
            w: 10,
            h: 10,
        }];

        let num_extensions = 1 + rand::random::<usize>() % 3;
        for _ in 0..num_extensions {
            let parent = house_rects[rand::random::<usize>() % house_rects.len()];
            let side = rand::random::<usize>() % 4;
            let ew = 6 + rand::random::<usize>() % 4;
            let eh = 6 + rand::random::<usize>() % 4;

            let ext = match side {
                0 => Rect {
                    x: parent.x + parent.w - 1,
                    y: parent.y,
                    w: ew,
                    h: eh,
                }, 
                1 => Rect {
                    x: parent.x.saturating_sub(ew) + 1,
                    y: parent.y,
                    w: ew,
                    h: eh,
                }, 
                2 => Rect {
                    x: parent.x,
                    y: parent.y + parent.h - 1,
                    w: ew,
                    h: eh,
                }, 
                _ => Rect {
                    x: parent.x,
                    y: parent.y.saturating_sub(eh) + 1,
                    w: ew,
                    h: eh,
                }, 
            };
            house_rects.push(ext);
        }

        let mut rooms = Vec::new();
        for rect in &house_rects {
            self.recursive_slice(rect.x, rect.y, rect.w, rect.h, &mut rooms);
        }

        for r in &house_rects {
            self.fill_rect(r.x, r.y, r.w, r.h, TILE_FLOOR);
        }

        for r in &house_rects {
            self.draw_rect_border(r.x, r.y, r.w, r.h);
        }

        for r in &rooms {
            self.draw_rect_border(r.rect.x, r.rect.y, r.rect.w, r.rect.h);
        }

        let adj = self.get_adjacency_map(&rooms);
        for (u, neighbors) in &adj {
            for v in neighbors {
                if u < v {
                    self.place_smart_door(&rooms[*u], &rooms[*v]);
                }
            }
        }

        self.place_outer_entrance(&house_rects[0]);

        if let Some(hub_idx) = self.find_highest_centrality(&rooms, &adj) {
            let hub = &rooms[hub_idx];
            let cx = hub.rect.x + hub.rect.w / 2;
            let cy = hub.rect.y + hub.rect.h / 2;
            self.safe_set_tile(cx, cy, 'K');
        }
    }

    fn place_outer_entrance(&mut self, core: &Rect) {
        let door_x = core.x + 1 + rand::random::<usize>() % (core.w - 2);
        if core.y < self.height && door_x < self.width {
            self.map[core.y][door_x] = TILE_DOOR;
        }
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

    fn recursive_slice(&self, x: usize, y: usize, w: usize, h: usize, rooms: &mut Vec<Room>) {
        let min_dim = 5;
        let can_split_v = w >= min_dim * 2 - 1;
        let can_split_h = h >= min_dim * 2 - 1;

        if can_split_v && (rand::random::<bool>() || !can_split_h) {
            let split = min_dim + rand::random::<usize>() % (w - min_dim);
            self.recursive_slice(x, y, split, h, rooms);
            self.recursive_slice(x + split - 1, y, w - split + 1, h, rooms);
        } else if can_split_h {
            let split = min_dim + rand::random::<usize>() % (h - min_dim);
            self.recursive_slice(x, y, w, split, rooms);
            self.recursive_slice(x, y + split - 1, w, h - split + 1, rooms);
        } else {
            rooms.push(Room {
                rect: Rect { x, y, w, h },
                id: rooms.len(),
                is_hallway: false,
            });
        }
    }

    fn get_adjacency_map(&self, rooms: &[Room]) -> std::collections::HashMap<usize, Vec<usize>> {
        let mut adj = std::collections::HashMap::new();
        for i in 0..rooms.len() {
            for j in i + 1..rooms.len() {
                if self.are_adjacent(&rooms[i].rect, &rooms[j].rect) {
                    adj.entry(i).or_insert_with(Vec::new).push(j);
                    adj.entry(j).or_insert_with(Vec::new).push(i);
                }
            }
        }
        adj
    }

    fn are_adjacent(&self, a: &Rect, b: &Rect) -> bool {
        let inter_x = a.x.max(b.x) < (a.x + a.w).min(b.x + b.w);
        let inter_y = a.y.max(b.y) < (a.y + a.h).min(b.y + b.h);

        let touch_x = (a.x + a.w - 1 == b.x) || (b.x + b.w - 1 == a.x);
        let touch_y = (a.y + a.h - 1 == b.y) || (b.y + b.h - 1 == a.y);

        (inter_x && touch_y) || (inter_y && touch_x)
    }

    fn find_highest_centrality(
        &self,
        rooms: &[Room],
        adj: &std::collections::HashMap<usize, Vec<usize>>,
    ) -> Option<usize> {
        let mut scores = vec![0; rooms.len()];
        for start_node in 0..rooms.len() {
            let mut q = VecDeque::new();
            q.push_back(start_node);
            let mut visited = HashSet::new();
            visited.insert(start_node);

            while let Some(curr) = q.pop_front() {
                if let Some(neighbors) = adj.get(&curr) {
                    for &next in neighbors {
                        if !visited.contains(&next) {
                            scores[curr] += 1;
                            visited.insert(next);
                            q.push_back(next);
                        }
                    }
                }
            }
        }
        scores
            .iter()
            .enumerate()
            .max_by_key(|&(_, &score)| score) 
            .map(|(i, _)| i)
    }

    fn place_smart_door(&mut self, r1: &Room, r2: &Room) {
        let a = &r1.rect;
        let b = &r2.rect;

        let x_overlap = a.x.max(b.x)..(a.x + a.w).min(b.x + b.w);
        let y_overlap = a.y.max(b.y)..(a.y + a.h).min(b.y + b.h);

        if x_overlap.len() > 2 && (a.y + a.h - 1 == b.y || b.y + b.h - 1 == a.y) {
            let py = if a.y + a.h - 1 == b.y { b.y } else { a.y };
            let px = x_overlap.start + 1 + rand::random::<usize>() % (x_overlap.len() - 2);
            self.safe_set_tile(px, py, TILE_DOOR);
        } else if y_overlap.len() > 2 && (a.x + a.w - 1 == b.x || b.x + b.w - 1 == a.x) {
            let px = if a.x + a.w - 1 == b.x { b.x } else { a.x };
            let py = y_overlap.start + 1 + rand::random::<usize>() % (y_overlap.len() - 2);
            self.safe_set_tile(px, py, TILE_DOOR);
        }
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
