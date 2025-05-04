use crate::entities::Living;
pub struct AI {
    pub model: String,
    pub version: String,
}

impl Living for AI {
    fn name(&self) -> &str {
        &self.model
    }

    fn live(&mut self) {
        println!("{} is running version {}!", self.model, self.version);
    }
}

impl Living for &mut AI {
    fn name(&self) -> &str {
        (*self).model.as_str()
    }
    fn live(&mut self) {
        (*self).live();
    }
}

impl AI {
    pub fn new<S: Into<String>>(model: S, version: S) -> Self {
        AI {
            model: model.into(),
            version: version.into(),
        }
    }
}
