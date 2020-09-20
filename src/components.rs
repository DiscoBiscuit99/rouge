use specs::prelude::*;
use specs_derive::*;
use rltk::RGB;

#[derive(Component)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: u32,
    pub dirty: bool,
}

