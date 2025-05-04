use crate::entities::Living;

pub struct World {
    pub name: String,
    pub inhabitants: Vec<Box<dyn Living>>,
}

impl World {
    pub fn run(&mut self) {
        for inhabitant in &mut self.inhabitants {
            inhabitant.live();
            inhabitant.die();
        }
    }
}
