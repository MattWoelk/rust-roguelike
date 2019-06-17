use specs::{ReadStorage, System, Join};

use super::components::{Position, PrintMeTag};

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
