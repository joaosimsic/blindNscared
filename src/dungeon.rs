    use crate::common::{Rect, Room, TILE_FLOOR, TILE_WALL};

pub struct World {
    pub map: Vec<Vec<char>>,
    pub rooms: Vec<Room>,
    pub width: usize,
    pub height: usize,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        World {
            map: vec![vec![TILE_WALL; width]; height],
            rooms: Vec::new(),
            width,
            height,
        }
    }

    pub fn generate(&mut self) {
        let root = Rect {
            x: 1,
            y: 1,
            w: self.width - 2,
            h: self.height - 2,
        };

        self.bsp_split(root);
        self.carve_corridors();
        self.place_exits();
    }

    fn bsp_split(&mut self, rect: Rect) {
        if rect.w < 10 || rect.h < 10 {
            self.carve_room(rect);
            return;
        }

        let vertical = rand::random::<bool>();

        if vertical {
            let split_x = rect.x + (2..rect.w - 2).next().unwrap_or(rect.w / 2);

            self.bsp_split(Rect {
                x: rect.x,
                y: rect.y,
                w: split_x - rect.x,
                h: rect.h,
            });

            self.bsp_split(Rect {
                x: split_x,
                y: rect.y,
                w: rect.x + rect.w - split_x,
                h: rect.h,
            });
        } else {
            let split_y = rect.y + (2..rect.h - 2).next().unwrap_or(rect.h / 2);
            self.bsp_split(Rect {
                x: rect.x,
                y: rect.y,
                w: rect.w,
                h: split_y - rect.y,
            });
            self.bsp_split(Rect {
                x: rect.x,
                y: split_y,
                w: rect.w,
                h: rect.y + rect.h - split_y,
            });
        }
    }

    fn carve_room(&mut self, rect: Rect) {
        let x = rect.x + rand::random::<usize>() % 2;
        let y = rect.y + rand::random::<usize>() % 2;
        let w = (rect.w - 2).max(3);
        let h = (rect.h - 2).max(3);

        let room = Room {
            x,
            y,
            width: w,
            height: h,
        };

        (0..h)
            .flat_map(|dy| (0..w).map(move |dx| (dx, dy)))
            .filter(|(dx, dy)| x + dx < self.width && y + dy < self.height)
            .for_each(|(dx, dy)| self.map[y + dy][x + dx] = TILE_FLOOR);

        self.rooms.push(room);
    }

    fn carve_corridors(&mut self) {
        for i in 0..self.rooms.len() - 1 {
            let (x1, y1) = (
                self.rooms[i].x + self.rooms[i].width / 2,
                self.rooms[i].y + self.rooms[i].height / 2,
            );
            let (x2, y2) = (
                self.rooms[i + 1].x + self.rooms[i + 1].width / 2,
                self.rooms[i + 1].y + self.rooms[i + 1].height / 2,
            );

            self.carve_horizontal_corridor(x1, x2, y1);
            self.carve_vertical_corridor(y1, y2, x2);
        }
    }

    fn carve_horizontal_corridor(&mut self, x1: usize, x2: usize, y: usize) {
        let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };

        (start..=end)
            .filter(|x| *x < self.width && y < self.height)
            .for_each(|x| self.map[y][x] = TILE_FLOOR);
    }

    fn carve_vertical_corridor(&mut self, y1: usize, y2: usize, x: usize) {
        let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        (start..=end)
            .filter(|y| *y < self.height && x < self.width)
            .for_each(|y| self.map[y][x] = TILE_FLOOR);
    }

    fn place_exits(&mut self) {
        if let Some(room) = self.rooms.first() {
            let x = room.x + rand::random::<usize>() % room.width;
            let y = room.y + rand::random::<usize>() % room.height;

            self.map[y][x] = TILE_EXIT;
        }

        if let Some(room) = self.rooms.last() {
            let x = room.x + rand::random::<usize>() % room.width;
            let y = room.y + rand::random::<usize>() % room.height;
            self.map[y][x] = TILE_EXIT;
        }
    }
}
