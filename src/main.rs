use specs::{World, Component, VecStorage, System, SystemData, ReadStorage};
use specs::world::Builder;
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

#[derive(Debug, Default)]
struct PrintMeTag;

impl Component for PrintMeTag {
    type Storage = specs::NullStorage<Self>;
}

struct PrintingSystem;
impl <'a> System<'a> for PrintingSystem {
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
impl <'a> System<'a> for NotPrintingSystem {
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

    let mut dispatcher = specs::DispatcherBuilder::new()
        .with(PrintingSystem, "print_sys", &[])
        .with(NotPrintingSystem, "not_print_sys", &["print_sys"])
        .build();

    dispatcher.setup(&mut world.res);

    world.create_entity()
        .with(Position {x: 10, y: 10})
        .with(PrintMeTag {})
        .build();

    world.create_entity()
        .with(Position {x: 20, y: 10})
        .build();

    world.create_entity()
        .with(Position {x: 30, y: 10})
        .build();

    dispatcher.dispatch(&mut world.res);

    let mut x = 10;
    let mut y = 10;

    while !root.window_closed() {
        root.clear();
        root.put_char(x, y, '@', BackgroundFlag::None);
        root.flush();
        root.wait_for_keypress(false);
        x += 1;
        y += 1;
    }
}
