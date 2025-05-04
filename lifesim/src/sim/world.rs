use crate::entities::Living;

pub struct World<'a> {
    pub name: String,
    pub inhabitants: Vec<Box<dyn Living + 'a>>,
}

impl<'a> World<'a> {
    pub fn run(&mut self) {
        for inhabitant in &mut self.inhabitants {
            inhabitant.live();
            inhabitant.die();
        }
    }

    pub fn new<S: Into<String>>(name: S, inhabitants: Vec<Box<dyn Living + 'a>>) -> Self {
        let world_name = name.into();
        println!("World: {}", world_name);
        World {
            name: world_name,
            inhabitants,
        }
    }

    pub fn add_inhabitant(&mut self, inhabitant: impl Living + 'a) {
        self.inhabitants.push(Box::new(inhabitant));
    }
}
