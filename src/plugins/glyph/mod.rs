use bevy::prelude::*;

pub mod components;
mod systems;

#[derive(Resource, Clone)]
pub struct Tileset {
    pub path: String,
    pub handle: Handle<TextureAtlas>,
}

#[derive(Resource, Clone)]
pub struct GlyphAtlas {
    pub tileset: String,
    pub glyph_size: Vec2,
    pub columns: usize,
    pub rows: usize,
    pub scale: f32,
}

pub struct GlyphPlugin {
    pub glyph_atlas: GlyphAtlas,
}

impl Plugin for GlyphPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, systems::setup);
        app.add_systems(Update, systems::update);

        app.insert_resource(self.glyph_atlas.clone());
        app.insert_resource(Tileset {
            path: self.glyph_atlas.tileset.clone(),
            handle: Handle::default(),
        });
    }
}