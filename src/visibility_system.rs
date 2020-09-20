use specs::prelude::*;
use super::{Viewshed, Position, Map, Player};
use rltk::{field_of_view, Point};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData =
        (WriteExpect<'a, Map>,
         Entities<'a>,
         WriteStorage<'a, Viewshed>,
         WriteStorage<'a, Position>,
         ReadStorage<'a, Player>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range as i32, &*map);
                viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width as i32 && p.y >= 0 && p.y < map.height as i32);

                // If this is the player, reveal what they can see.
                let _p: Option<&Player> = player.get(ent);
                if let Some(_p) = _p {
                    for t in map.visible_tiles.iter_mut() { *t = false; }
                    for vis in viewshed.visible_tiles.iter() {
                        let i = map.xy_idx(vis.x as i16, vis.y as i16);
                        map.revealed_tiles[i] = true;
                        map.visible_tiles[i] = true;
                    }
                }
            }
        }
    }
}
