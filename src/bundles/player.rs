use bevy::prelude::*;

use crate::{components::{tags::Player, position::Position}, plugins::glyph::components::Glyph};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub tag: Player,
    pub position: Position,
    pub glyph: Glyph,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        let position = Position { x: 0, y: 0 };

        Self {
            tag: Player,
            position: position.clone(),
            glyph: Glyph {
                index: '@' as usize,
                fg: Color::WHITE,
                x: position.x,
                y: position.y,
                ..default()
            },
        }
    }
}
