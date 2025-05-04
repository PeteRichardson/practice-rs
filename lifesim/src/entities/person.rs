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

impl<'a> Living for &'a mut Person {
    fn name(&self) -> &str {
        (*self).name.as_str()
    }
    fn live(&mut self) {
        (*self).live();
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
