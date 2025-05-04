pub mod entities;
pub mod sim;

use entities::{ai::AI, person::Person};
use sim::World;

fn main() {
    let mut world = World::new("Simulated World", vec![]);
    world.add_inhabitant(Person::new("Pete", 59));
    world.add_inhabitant(AI::new("GPT-4", "4.0"));

    world.run();
}
