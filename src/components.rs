use specs::{Component, VecStorage};

#[derive(Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Default, Component)]
#[storage(VecStorage)]
pub struct PrintMeTag;
