use bevy::prelude::Color;

pub enum Palette {}

impl Palette {
    pub const RED: Color = Color::rgb(0.792, 0.337, 0.262);
    pub const ORANGE: Color = Color::rgb(0.909, 0.525, 0.305);
    pub const PEACH: Color = Color::rgb(0.945, 0.729, 0.556);
    pub const FLAX: Color = Color::rgb(0.925, 0.850, 0.600);
    pub const WHITE: Color = Color::rgb(0.945, 0.941, 0.850);
    pub const TEAL: Color = Color::rgb(0.647, 0.858, 0.815);
    pub const STEEL: Color = Color::rgb(148.0, 0.756, 0.729);
    pub const BLUE: Color = Color::rgb(0.305, 0.662, 0.756);
    pub const GRAY: Color = Color::rgb(0.215, 0.231, 0.258);
    pub const LAVENDER: Color = Color::rgb(0.517, 0.482, 0.560);
    pub const BROWN: Color = Color::rgb(0.427, 0.258, 0.243);
    pub const TAN: Color = Color::rgb(0.560, 0.396, 0.250);
    pub const GOLD: Color = Color::rgb(0.666, 0.521, 0.215);
    pub const YELLOW: Color = Color::rgb(0.905, 0.709, 0.313);
    pub const SAGE: Color = Color::rgb(0.643, 0.678, 0.403);
    pub const GREEN: Color = Color::rgb(0.392, 0.580, 0.317);
    pub const SEA: Color = Color::rgb(0.243, 0.419, 0.352);
    pub const BLACK: Color = Color::rgb(0.137, 0.137, 0.137);
}
