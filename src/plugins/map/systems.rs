use bevy::prelude::*;
use rand_seeder::rand_core::RngCore;
use rand_seeder::Seeder;
use rand_pcg::Pcg64;

use crate::plugins::glyph::components::Glyph;
use crate::components::position::Position;
use crate::palette::Palette;

use super::bundles::TileBundle;
use super::{MapConfig, Chunk, BiomeType};

pub fn generate_map(mut commands: Commands, map_config: Res<MapConfig>) {
    let mut random: Pcg64 = Seeder::from(map_config.seed.clone()).make_rng();

    for x in 0..map_config.width {
        for y in 0..map_config.height {
            let random_number = random.next_u32() % 100;

            let biome = if random_number < 60 {
                BiomeType::Forest
            } else {
                BiomeType::Field
            };

            let chunk = Chunk {
                biome,
                size: 16,
                x,
                y,
            };

            generate_chunk(&mut commands, chunk, &mut random);
        }
    }
}

pub fn generate_chunk(commands: &mut Commands, chunk: Chunk, rng: &mut Pcg64) {
    match chunk.biome {
        BiomeType::Forest => generate_forest_biome(commands, chunk, rng),
        BiomeType::Field => generate_field_biome(commands, chunk),
    }
}

pub fn generate_forest_biome(commands: &mut Commands, chunk: Chunk, rng: &mut Pcg64) {
    for x in 0..chunk.size {
        for y in 0..chunk.size {
            let random_number = rng.next_u32() % 100;

            if random_number < 10 {
                commands.spawn(TileBundle {
                    glyph: Glyph {
                        index: 't' as usize,
                        fg: Palette::GREEN,
                        ..default()
                    },
                    position: Position {
                        x: (chunk.size * chunk.x) + x,
                        y: (chunk.size * chunk.y) + y,
                    },
                });
                continue;
            } else {
                commands.spawn(TileBundle {
                    glyph: Glyph {
                        index: '.' as usize,
                        fg: Palette::SAGE,
                        ..default()
                    },
                    position: Position {
                        x: (chunk.size * chunk.x) + x,
                        y: (chunk.size * chunk.y) + y,
                    },
                });
            }
        }
    }
}

#[allow(dead_code)]
pub fn generate_field_biome(commands: &mut Commands, chunk: Chunk) {
    for x in 0..chunk.size {
        for y in 0..chunk.size {
            commands.spawn(TileBundle {
                glyph: Glyph {
                    index: '.' as usize,
                    fg: Palette::SAGE,
                    ..default()
                },
                position: Position {
                    x: (chunk.size * chunk.x) + x,
                    y: (chunk.size * chunk.y) + y,
                },
            });
        }
    }
}
