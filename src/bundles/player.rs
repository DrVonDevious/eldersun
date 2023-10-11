use bevy::prelude::*;

use crate::{components::{tags::Player, position::Position}, plugins::{glyph::components::Glyph, action_queue::components::ActionQueue}, palette::Palette};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub tag: Player,
    pub position: Position,
    pub glyph: Glyph,
    pub action_queue: ActionQueue,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        let position = Position { x: 0, y: 0 };

        Self {
            tag: Player,
            position: position.clone(),
            glyph: Glyph {
                index: '@' as usize,
                fg: Palette::WHITE,
                x: position.x,
                y: position.y,
                ..default()
            },
            action_queue: ActionQueue::with_interval(0.2),
        }
    }
}
