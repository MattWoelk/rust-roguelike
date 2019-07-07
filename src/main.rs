#![warn(rust_2018_idioms)]

use specs::world::Builder;
use specs::World;

mod components;
use components::{CharacterGlyph, PlayerController, Position, PrintMeTag};

mod systems;
use systems::{NotPrintingSystem, PlayerMove, PrintingSystem};

mod vulkansystem;
use vulkansystem::VulkanTriangleRenderer;
use winit::VirtualKeyCode;

#[derive(Debug, Default)]
pub struct GameState {
    end: bool,
    key_press: Option<VirtualKeyCode>,
}

fn main() {
    let mut world = World::new();
    world.add_resource(GameState::default());

    let mut dispatcher = specs::DispatcherBuilder::new()
        .with_thread_local(VulkanTriangleRenderer::new())
        //.with_thread_local(Render { window: root })
        //.with(PrintingSystem, "print_sys", &[])
        //.with(NotPrintingSystem, "not_print_sys", &["print_sys"])
        .with(PlayerMove, "player_move", &[])
        .build();

    dispatcher.setup(&mut world.res);

    world.register::<Position>();
    world.register::<PrintMeTag>();
    world.register::<CharacterGlyph>();

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
