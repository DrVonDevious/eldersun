use bevy::prelude::*;

#[derive(Component)]
pub struct Glyph {
    pub index: usize,
    pub color: Color,
    pub x: u32,
    pub y: u32,
}