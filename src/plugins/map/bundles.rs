use bevy::prelude::*;

use crate::components::position::Position;
use crate::plugins::glyph::components::Glyph;

#[derive(Bundle)]
pub struct TileBundle {
    pub position: Position,
    pub glyph: Glyph,
}
