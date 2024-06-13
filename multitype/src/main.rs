mod pets;
use pets::{Cat, Dog, Speak};
fn main() {
    let bella = Cat::new("Bella", 12);
    let peaches = Dog {
        name: "Peaches".to_string(),
        age: 9,
    };
    let speaking_pets: Vec<Box<dyn Speak>> = vec![Box::new(bella), Box::new(peaches)];

    for pet in speaking_pets {
        pet.speak();
    }
}
