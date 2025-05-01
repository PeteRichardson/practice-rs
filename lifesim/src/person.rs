use crate::living::Living;
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
    pub fn create(name: &str, age: u32) -> Box<dyn Living> {
        Box::new(Person {
            name: name.into(),
            age,
        })
    }
}
