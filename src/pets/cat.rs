use crate::pets::Speak;
pub struct Cat<'a> {
    pub name: &'a str,
    pub age: u8,
    pub litter_box: &'a str,
}

impl<'a> Cat<'a> {
    pub fn new(name: &'a str, age: u8) -> Self {
        let mut litter_box = "Blue Plastic";
        if name == "Stripes" {
            litter_box = "Gray Steel";
        }
        Cat {
            name: name,
            age: age,
            litter_box: litter_box,
        }
    }
}

impl Speak for Cat<'_> {
    fn speak(&self) {
        println!(
            "Meow! My name is {}. I am {} years old, and my litter box is {}",
            self.name, self.age, self.litter_box
        );
    }
}
