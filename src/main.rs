use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;

mod components;
use components::*;

mod player;
use player::*;

mod map;
use map::*;

mod visibility_system;
use visibility_system::*;

///// CONSTS /////

pub const SIZE: u16 = 80;

///// STRUCTS /////

pub struct State {
    world: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.world);
        self.world.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        program_controls(ctx);

        player_move(self, ctx);

        self.run_systems();

        // Render the map
        draw_map(&self.world, ctx);

        // Render the renderables
        let map = self.world.fetch::<Map>();
        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            let i = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[i] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

///// FUNCTIONS /////

fn program_controls(ctx: &mut Rltk) {
    match ctx.key {
        Some(key) => match key {
            VirtualKeyCode::Escape => ctx.quit(),
            _ => {},
        },
        _ => {},
    }
}


/////////////////

fn main() -> rltk::BError { 
    // INIT //
    use rltk::RltkBuilder;
    let mut ctx = RltkBuilder::simple(SIZE, SIZE)
        .unwrap()
        .with_title("Rogue")
        .build()?;

    //ctx.post_scanlines = true;
    //ctx.post_screenburn = true;
    //ctx.with_post_scanlines(false);

    let mut gs = State { world: World::new() };

    let map = Map::new();
    let (player_x, player_y) = map.rooms[0].center();

    // REGISTERING // 
    gs.world.register::<Position>();
    gs.world.register::<Renderable>();
    gs.world.register::<Player>();
    gs.world.register::<Viewshed>();

    // The player
    gs.world.create_entity()
        .with(Position { 
            x: player_x, 
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::WHITE),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed { 
            visible_tiles: Vec::new(), 
            range: 8, 
            dirty: true 
        }).build();

    // Goblins
    let mut rng = rltk::RandomNumberGenerator::new();
    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();
        
        let glyph: rltk::FontCharType;
        let fg: RGB;

        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => { 
                glyph = rltk::to_cp437('g');
                fg = RGB::named(rltk::YELLOWGREEN);
            },
            _ => { 
                glyph = rltk::to_cp437('o');
                fg = RGB::named(rltk::BROWN2);
            },
        }

        gs.world.create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph: glyph,
                fg,
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed { 
                visible_tiles: Vec::new(), 
                range: 8, 
                dirty: true 
            }).build();
    }

    gs.world.insert(map);

    // RUNNING // 
    rltk::main_loop(ctx, gs)
}

