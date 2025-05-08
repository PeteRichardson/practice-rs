use crate::entities::Living;
use rand;

pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Living for Person {
    fn name(&self) -> &str {
        &self.name
    }

    fn live(&mut self) {
        println!("{} was born!", self.name);
        self.age = rand::random::<u32>() % 100;
        println!("{} is {} years old!", self.name, self.age);
    }
}

impl Person {
    pub fn new(name: &str, age: u32) -> Person {
        Person {
            name: name.into(),
            age,
        }
    }
}

impl Clone for Person {
    fn clone(&self) -> Self {
        println!(
            "CLONED {} at {:?}",
            self.name,
            std::panic::Location::caller()
        );
        Person {
            name: self.name.clone(),
            age: self.age,
        }
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        println!(
            "DROPPED {} at {:?}",
            self.name,
            std::panic::Location::caller()
        );
    }
}
