use specs::world::Builder;
use specs::World;
use tcod::colors;
use tcod::console::*;
use tcod::input::KeyCode;

mod components;
use components::{CharacterGlyph, PlayerController, Position, PrintMeTag};

mod systems;
use systems::{NotPrintingSystem, PlayerMove, PrintingSystem, Render};

mod vulkansystem;
use vulkansystem::VulkanTriangleRenderer;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

#[derive(Debug, Default)]
pub struct GameState {
    end: bool,
    key_press: Option<KeyCode>,
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
        .with_thread_local(VulkanTriangleRenderer::new())
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

    //main_vulkano();
}
