use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}