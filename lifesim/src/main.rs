pub mod entities;
pub mod sim;

use entities::{ai::AI, person::Person};
use sim::World;

fn main() {
    let living_things = vec![Person::create("Pete", 59), AI::create("GPT-3", "3.5")];

    let mut world = World {
        name: "My World".to_string(),
        inhabitants: living_things,
    };
    println!("World Name: {}", world.name);
    world.run();
}
