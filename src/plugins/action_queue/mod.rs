use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct ActionQueuePlugin;

impl Plugin for ActionQueuePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::update);
    }
}