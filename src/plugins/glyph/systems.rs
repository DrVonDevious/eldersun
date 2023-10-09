use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use super::{Tileset, GlyphAtlas, components::Glyph};

pub fn setup(
    asset_loader: Res<AssetServer>,
    mut tileset: ResMut<Tileset>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    glyph_atlas: ResMut<GlyphAtlas>,
) {
    let texture_handle: Handle<Image> = asset_loader.load(tileset.path.as_str());
    let atlas = TextureAtlas::from_grid(
        texture_handle,
        glyph_atlas.glyph_size,
        glyph_atlas.columns,
        glyph_atlas.rows,
        None,
        None,
    );

    tileset.handle = atlases.add(atlas);
}

pub fn update(
    mut commands: Commands,
    glyph_atlas: Res<GlyphAtlas>,
    tileset: Res<Tileset>,
    query: Query<(Entity, &Glyph)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, glyph) in query.iter() {
        let background_color = match glyph.bg {
            Some(color) => color,
            None => glyph_atlas.background_color,
        };

        // commands.entity(entity).insert(MaterialMesh2dBundle {
        //     mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        //     material: materials.add(ColorMaterial::from(background_color)),
        //     transform: Transform::from_scale(Vec3::splat(12.0)),
        //     ..default()
        // });

        commands.entity(entity).insert(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: glyph.index,
                color: glyph.fg,
                ..default()
            },
            texture_atlas: tileset.handle.clone(),
            transform: Transform::from_translation(
                Vec3::new(
                    glyph.x as f32 * glyph_atlas.glyph_size.x * glyph_atlas.scale,
                    glyph.y as f32 * glyph_atlas.glyph_size.y * glyph_atlas.scale,
                    0.0,
                )
            ).with_scale(Vec3::new(glyph_atlas.scale, glyph_atlas.scale, 1.0)),
            ..default()
        });
    }
}