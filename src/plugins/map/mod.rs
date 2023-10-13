use bevy::prelude::*;

pub mod bundles;
mod systems;

pub enum BiomeType {
    Forest,
    Field,
}

pub struct Chunk {
    biome: BiomeType,
    size: i32,
    x: i32,
    y: i32,
}

pub struct MapPlugin {
    pub map_config: MapConfig,
}

#[derive(Resource, Clone)]
pub struct MapConfig {
    pub width: i32,
    pub height: i32,
    pub seed: String,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::generate_map);
        app.insert_resource(self.map_config.clone());
    }
}
