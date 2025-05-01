use crate::living::Living;

pub struct World {
    pub name: String,
    pub inhabitants: Vec<Box<dyn Living>>,
}

impl World {
    pub fn run(&mut self) {
        for mut inhabitant in self.inhabitants {
            inhabitant.live();
            inhabitant.die();
        }
    }
}
