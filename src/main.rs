use specs::world::Builder;
use specs::{Component, Join, Read, ReadStorage, System, VecStorage, World, WriteStorage};
use tcod::colors;
use tcod::console::*;
use tcod::input::{Key, KeyCode};

#[macro_use]
extern crate specs_derive;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

#[derive(Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
struct CharacterGlyph {
    glyph: char,
}

#[derive(Debug, Default, Component)]
#[storage(VecStorage)]
struct PrintMeTag;

#[derive(Debug, Default)]
struct PlayerController;
impl Component for PlayerController {
    type Storage = specs::NullStorage<Self>;
}

#[derive(Debug, Default)]
struct GameState {
    end: bool,
    key_press: Option<KeyCode>,
}

struct PrintingSystem;
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

struct NotPrintingSystem;
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

struct Render {
    window: Root,
}

impl<'a> System<'a> for Render {
    type SystemData = (
        ReadStorage<'a, CharacterGlyph>,
        ReadStorage<'a, Position>,
        specs::Write<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use KeyCode::*;

        let root = &mut self.window;

        let (sprites, positions, mut game_state) = data;

        root.clear();
        for (sprite, pos) in (&sprites, &positions).join() {
            root.put_char(pos.x, pos.y, sprite.glyph, BackgroundFlag::None);
        }
        root.flush();
        let key = root.wait_for_keypress(false);

        let key_press = match key {
            Key { code: Escape, .. } => {
                game_state.end = true;
                None
            }
            Key { code: Up, .. } => Some(Up),
            Key { code: Down, .. } => Some(Down),
            Key { code: Left, .. } => Some(Left),
            Key { code: Right, .. } => Some(Right),
            _ => None,
        };

        game_state.key_press = key_press;
        game_state.end = root.window_closed();
    }
}

struct PlayerMove;
impl<'a> System<'a> for PlayerMove {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, PlayerController>,
        Read<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use KeyCode::*;
        let (mut position, player_controlled, game_state) = data;
        if let Some(key) = game_state.key_press {
            for (pos, _) in (&mut position, &player_controlled).join() {
                match key {
                    Up => pos.y -= 1,
                    Down => pos.y += 1,
                    Left => pos.x -= 1,
                    Right => pos.x += 1,
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    let mut root = Root::initializer()
        .font("terminal.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Roguelike using specs")
        .init();

    root.set_default_foreground(colors::WHITE);

    tcod::system::set_fps(LIMIT_FPS);

    let mut world = World::new();
    world.add_resource(GameState::default());

    let mut dispatcher = specs::DispatcherBuilder::new()
        .with(PrintingSystem, "print_sys", &[])
        .with(NotPrintingSystem, "not_print_sys", &["print_sys"])
        .with(PlayerMove, "player_move", &[])
        .with_thread_local(Render { window: root })
        .build();

    dispatcher.setup(&mut world.res);

    world
        .create_entity()
        .with(Position { x: 10, y: 10 })
        .with(PrintMeTag {})
        .with(CharacterGlyph { glyph: 'y' })
        .with(PlayerController {})
        .build();

    world
        .create_entity()
        .with(Position { x: 20, y: 10 })
        .with(CharacterGlyph { glyph: 'o' })
        .build();

    world
        .create_entity()
        .with(Position { x: 30, y: 10 })
        .with(CharacterGlyph { glyph: 'T' })
        .build();

    loop {
        dispatcher.dispatch(&world.res);
        let game_state = world.read_resource::<GameState>();
        if game_state.end {
            break;
        }
    }
}
