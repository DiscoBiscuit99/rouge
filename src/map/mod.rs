use rltk::{RGB, Rltk, RandomNumberGenerator, BaseMap, Algorithm2D, Point};
use specs::prelude::*;
use specs_derive::*;
use std::cmp::{max, min};

mod rect;
use rect::Rect;

use super::SIZE;

//// ENUMS ////

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Floor,
    Wall,
    Grass,
    Foliage,
}

//// STRUCTS ////

#[derive(Default)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: u16,
    pub height: u16,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}
 
impl Map {
    pub fn xy_idx(&self, x: i16, y: i16) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn apply_room(&mut self, room: &Rect) {
        let mut rng = RandomNumberGenerator::new();

        for y in room.y+1 .. room.y+room.h+1 {
            for x in room.x+1 .. room.x+room.w+1 {
                let i = self.xy_idx(x, y);
                let roll = rng.range(0, 10);
                if roll == 1 {
                    self.tiles[i] = TileType::Foliage;
                } else if roll == 2 || roll == 3 || roll == 4 {
                    self.tiles[i] = TileType::Grass;
                } else {
                    self.tiles[i] = TileType::Floor;
                }
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i16, x2: i16, y: i16) {
        for x in min(x1, x2)..=max(x1, x2) {
            let i = self.xy_idx(x, y);
            if i > 0 && i < self.width as usize * self.height as usize {
                self.tiles[i as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i16, y2: i16, x: i16) {
        for y in min(y1, y2)..=max(y1, y2) {
            let i = self.xy_idx(x, y);
            if i > 0 && i < self.width as usize * self.height as usize {
                self.tiles[i as usize] = TileType::Floor;
            }
        }
    }

    pub fn new() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; SIZE as usize * SIZE as usize],
            rooms: Vec::new(),
            width: SIZE,
            height: SIZE,
            revealed_tiles: vec![false; SIZE as usize * SIZE as usize],
            visible_tiles: vec![false; SIZE as usize * SIZE as usize],
        };

        const MIN_ROOMS: i32 =  10;
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 =  5;
        const MAX_SIZE: i32 = 20;

        let mut rng = RandomNumberGenerator::new();

        for _ in MIN_ROOMS..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width as i32 - w - 2);
            let y = rng.roll_dice(1, map.height as i32 - h - 2);

            let new_room = Rect::new(x as i16, y as i16, w as i16, h as i16);

            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersects(other_room) { ok = false; }
            }
        
            if ok {
                map.apply_room(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len()-1].center();
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, i: usize) -> bool {
        self.tiles[i] == TileType::Wall ||
        self.tiles[i] == TileType::Foliage
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

pub fn draw_map(world: &World, ctx: &mut Rltk) {
    let map = world.fetch::<Map>();

    let mut x = 0;
    let mut y = 0;

    for (i, tile) in map.tiles.iter().enumerate() {
        // Render a tile depending upon the tile type
        if map.revealed_tiles[i] {
            let glyph;
            let mut fg;
            let bg;
            match tile {
                TileType::Floor => {
                    glyph = rltk::to_cp437('.');
                    fg = RGB::from_f32(0.0, 0.5, 0.5);
                    bg = RGB::from_f32(0., 0., 0.);
                },
                TileType::Wall => {
                    glyph = rltk::to_cp437('■');
                    fg = RGB::from_f32(0., 0.5, 0.5);
                    bg = RGB::from_f32(0., 0., 0.);
                },
                TileType::Grass => {
                    glyph = rltk::to_cp437('"');
                    fg = RGB::named(rltk::FORESTGREEN);
                    bg = RGB::from_f32(0., 0., 0.);
                },
                TileType::Foliage => {
                    //glyph = rltk::to_cp437('♣');
                    glyph = rltk::to_cp437('φ');
                    fg = RGB::named(rltk::FORESTGREEN);
                    bg = RGB::named(rltk::BLACK);
                }
            }
            if !map.visible_tiles[i] { fg = fg.to_greyscale(); }
            ctx.set(x, y, fg, bg, glyph);
        }

        x += 1;
        if x > SIZE-1 {
            x = 0;
            y += 1;
        }
    }
}

