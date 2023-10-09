use bevy::prelude::*;

use crate::{plugins::glyph::components::Glyph, components::position::Position};

pub fn update(
    mut query: Query<(&mut Glyph, &Position)>,
) {
    for (mut glyph, position) in query.iter_mut() {
        glyph.x = position.x;
        glyph.y = position.y;
    }
}