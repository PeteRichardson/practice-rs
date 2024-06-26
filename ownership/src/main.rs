// Running through some examples from chapter 4 of
// The Rust Programming Language, 2nd ed.

use std::fmt::Display;

struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    fn append_s(&mut self) {
        self.name.push('s')
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.name, self.age)
    }
}


fn main() {
    let mut people = [
        Person::new("Pete".to_string(), 40),
        Person::new("Wendy".to_string(), 40),
        Person::new("Kat".to_string(), 10),
        Person::new("Bella".to_string(), 11),
    ];
    people.iter_mut().for_each(|p| {
        p.append_s();
        println!("person: {}", p)
    });
}
