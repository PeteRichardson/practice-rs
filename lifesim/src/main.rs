pub mod ai;
pub mod living;
pub mod person;
pub mod world;

use ai::AI;
use person::Person;

fn main() {
    let world = world::World {
        name: String::from("My World"),
        inhabitants: vec![Person::create("Pete", 59), AI::create("GPT-3", "3.5")],
    };
    println!("World Name: {}", world.name);
    for mut entity in world.inhabitants {
        entity.live();
        entity.die();
    }
}
