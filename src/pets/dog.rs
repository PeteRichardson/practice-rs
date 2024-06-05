use crate::pets::Speak;

pub struct Dog {
    pub name: String,
    pub age: u8,
}

impl Speak for Dog {
    fn speak(&self) {
        println!(
            "Woof! My name is {} and I am {} years old!",
            self.name, self.age
        );
    }
}
