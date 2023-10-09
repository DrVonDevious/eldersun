use bevy::prelude::*;

#[derive(Component)]
pub struct Glyph {
    pub index: usize,
    pub fg: Color,
    pub bg: Option<Color>,
    pub x: i32,
    pub y: i32,
}

impl Default for Glyph {
    fn default() -> Self {
        Self {
            index: 0,
            fg: Color::WHITE,
            bg: None,
            x: 0,
            y: 0,
        }
    }
}