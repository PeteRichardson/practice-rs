pub mod ai;
pub mod living;
pub mod person;
pub mod world;

fn main() {
    let world = world::World {
        name: String::from("My World"),
        inhabitants: vec![
            Box::new(person::Person {
                age: 59,
                name: "Pete".into(),
            }),
            Box::new(ai::AI {
                model: "GPT-4".into(),
                version: "4.0".into(),
            }),
        ],
    };
    println!("World Name: {}", world.name);
    for mut entity in world.inhabitants {
        entity.live();
        entity.die();
    }
}
