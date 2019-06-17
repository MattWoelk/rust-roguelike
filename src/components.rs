use specs::{Component, NullStorage, VecStorage};
use specs_derive::Component;

#[derive(Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Default, Component)]
#[storage(VecStorage)]
pub struct PrintMeTag;

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct CharacterGlyph {
    pub glyph: char,
}

#[derive(Debug, Default, Component)]
#[storage(NullStorage)]
pub struct PlayerController;
