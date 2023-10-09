use bevy::prelude::Color;

pub enum Palette {}

impl Palette {
    pub const RED: Color = Color::rgb(202.0, 86.0, 62.0);
    pub const ORANGE: Color = Color::rgb(232.0, 134.0, 78.0);
    pub const PEACH: Color = Color::rgb(241.0, 186.0, 142.0);
    pub const FLAX: Color = Color::rgb(236.0, 217.0, 153.0);
    pub const WHITE: Color = Color::rgb(241.0, 240.0, 217.0);
    pub const TEAL: Color = Color::rgb(165.0, 219.0, 208.0);
    pub const STEEL: Color = Color::rgb(148.0, 193.0, 186.0);
    pub const BLUE: Color = Color::rgb(78.0, 169.0, 193.0);
    pub const GRAY: Color = Color::rgb(55.0, 59.0, 66.0);
    pub const LAVENDER: Color = Color::rgb(132.0, 123.0, 143.0);
    pub const BROWN: Color = Color::rgb(109.0, 66.0, 62.0);
    pub const TAN: Color = Color::rgb(143.0, 101.0, 64.0);
    pub const GOLD: Color = Color::rgb(170.0, 133.0, 55.0);
    pub const YELLOW: Color = Color::rgb(231.0, 181.0, 80.0);
    pub const SAGE: Color = Color::rgb(164.0, 173.0, 103.0);
    pub const GREEN: Color = Color::rgb(100.0, 148.0, 81.0);
    pub const SEA: Color = Color::rgb(62.0, 107.0, 90.0);
    pub const BLACK: Color = Color::rgb(35.0, 35.0, 35.0);
}