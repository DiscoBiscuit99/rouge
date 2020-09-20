use rltk::{VirtualKeyCode, Rltk};
use specs::prelude::*;
use super::{Position, Player, Map, TileType, State, SIZE, Viewshed};
use std::cmp::{min, max};

pub fn try_move_player(dx: i16, dy: i16, world: &mut World) {
    let mut positions = world.write_storage::<Position>();
    let mut players = world.write_storage::<Player>();
    let mut viewsheds = world.write_storage::<Viewshed>();

    let mut map = world.fetch_mut::<Map>();

    for (_, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let dest_idx = map.xy_idx(pos.x + dx, pos.y + dy);
        if map.tiles[dest_idx] != TileType::Wall {
            pos.x = min((SIZE-1 as u16) as i16, max(0, pos.x + dx));
            pos.y = min((SIZE-1 as u16) as i16, max(0, pos.y + dy));

            viewshed.dirty = true;

            if map.tiles[dest_idx] == TileType::Foliage {
                map.tiles[dest_idx] = TileType::Floor;
            }
        }
    }
}

pub fn player_move(gs: &mut State, ctx: &mut Rltk) {
    // Move the player
    match ctx.key {
        Some(key) => match key {
            VirtualKeyCode::H | 
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.world),
            VirtualKeyCode::L |
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.world),
            VirtualKeyCode::K | 
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.world),
            VirtualKeyCode::J | 
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.world),
            _ => {},
        },
        _ => {},
    }
}

