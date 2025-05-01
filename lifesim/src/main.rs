pub mod ai;
pub mod living;
pub mod person;
pub mod world;

use ai::AI;
use person::Person;

fn main() {
    let living_things = vec![Person::create("Pete", 59), AI::create("GPT-3", "3.5")];

    let world = world::World {
        name: String::from("My World"),
        inhabitants: living_things,
    };
    println!("World Name: {}", world.name);
    world.run();
}
