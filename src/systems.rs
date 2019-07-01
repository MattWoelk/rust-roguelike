use specs::{Join, Read, ReadStorage, System, WriteStorage};
use tcod::console::*;
use tcod::input::KeyCode;

use crate::components::{CharacterGlyph, PlayerController, Position, PrintMeTag};
use crate::GameState;

pub struct PrintingSystem;
impl<'a> System<'a> for PrintingSystem {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, PrintMeTag>);

    fn run(&mut self, data: Self::SystemData) {
        println!("Running print system");
        let (position, print_me) = data;
        for (pos, _) in (&position, &print_me).join() {
            println!("{:?}", pos);
        }
    }
}

pub struct NotPrintingSystem;
impl<'a> System<'a> for NotPrintingSystem {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, PrintMeTag>);

    fn run(&mut self, data: Self::SystemData) {
        println!("Running NOT print system");
        let (position, print_me) = data;
        for (pos, _) in (&position, !&print_me).join() {
            println!("{:?}", pos);
        }
    }
}

pub struct Render {
    pub window: Root,
}
impl<'a> System<'a> for Render {
    type SystemData = (
        ReadStorage<'a, CharacterGlyph>,
        ReadStorage<'a, Position>,
        specs::Write<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let root = &mut self.window;

        let (sprites, positions, mut game_state) = data;

        //game_state.end = false;

        root.clear();
        for (sprite, pos) in (&sprites, &positions).join() {
            root.put_char(pos.x, pos.y, sprite.glyph, BackgroundFlag::None);
        }
        root.flush();
        //let key = root.wait_for_keypress(false);

        //let key_press = match key.code {
        //    KeyCode::Escape => {
        //        game_state.end = true;
        //        None
        //    }
        //    key => Some(key),
        //};

        //game_state.key_press = key_press;
        //game_state.end |= root.window_closed();
    }
}

pub struct PlayerMove;
impl<'a> System<'a> for PlayerMove {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, PlayerController>,
        Read<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut position, player_controlled, game_state) = data;
        if let Some(key) = game_state.key_press {
            for (pos, _) in (&mut position, &player_controlled).join() {
                match key {
                    KeyCode::Up => pos.y -= 1,
                    KeyCode::Down => pos.y += 1,
                    KeyCode::Left => pos.x -= 1,
                    KeyCode::Right => pos.x += 1,
                    _ => {}
                }
            }
        }
    }
}
