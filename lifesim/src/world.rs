use crate::living::Living;

pub struct World {
    pub name: String,
    pub inhabitants: Vec<Box<dyn Living>>,
}
