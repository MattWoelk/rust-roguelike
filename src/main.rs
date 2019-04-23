use specs::world::Builder;
use specs::{Component, ReadStorage, System, SystemData, VecStorage, World};
use tcod::colors;
use tcod::console::*;
//use proc_macro::Delimiter;

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

struct CharacterGlyph {
    glyph: char,
}

impl Component for CharacterGlyph {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Default)]
struct PrintMeTag;

impl Component for PrintMeTag {
    type Storage = specs::NullStorage<Self>;
}

#[derive(Debug, Default)]
struct GameState {
    end: bool,
}

struct PrintingSystem;
impl<'a> System<'a> for PrintingSystem {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, PrintMeTag>);

    fn run(&mut self, data: Self::SystemData) {
        println!("Running print system");
        use specs::Join;
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
        use specs::Join;
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
        use specs::Join;

        let root = &mut self.window;

        let (sprites, positions, mut game_state) = data;

        root.clear();
        for (sprite, pos) in (&sprites, &positions).join() {
            root.put_char(pos.x, pos.y, sprite.glyph, BackgroundFlag::None);
        }
        root.flush();
        root.wait_for_keypress(false);

        game_state.end = root.window_closed();
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
    world.add_resource(GameState { end: false });

    let mut dispatcher = specs::DispatcherBuilder::new()
        .with(PrintingSystem, "print_sys", &[])
        .with(NotPrintingSystem, "not_print_sys", &["print_sys"])
        .with_thread_local(Render { window: root })
        .build();

    dispatcher.setup(&mut world.res);

    world
        .create_entity()
        .with(Position { x: 10, y: 10 })
        .with(PrintMeTag {})
        .with(CharacterGlyph { glyph: 'y' })
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
        dispatcher.dispatch(&mut world.res);
        let game_state = world.read_resource::<GameState>();
        if game_state.end {
            break;
        }
    }
}
