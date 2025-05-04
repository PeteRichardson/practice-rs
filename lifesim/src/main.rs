pub mod entities;
pub mod sim;

use entities::{ai::AI, person::Person};
use sim::World;

fn main() {
    let mut john = Person::new("John", 30);
    let mut gpt = AI::new("GPT9", "9.0");
    let mut world = World::new("Simulated World", vec![]);

    world.add_inhabitant(Person::new("Pete", 59));
    world.add_inhabitant(&mut john);
    world.add_inhabitant(&mut gpt);

    world.run();
    // world destroyed
    // john destroyed
}
