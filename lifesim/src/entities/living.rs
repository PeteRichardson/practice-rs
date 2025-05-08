pub trait Living {
    fn name(&self) -> &str;
    fn live(&mut self);
    fn die(&self) {
        println!("{} has died!", self.name());
    }
}

// Implement &mut Living for all types that implement Living
impl<T: Living> Living for &mut T {
    fn name(&self) -> &str {
        (**self).name()
    }

    fn live(&mut self) {
        (**self).live()
    }

    fn die(&self) {
        (**self).die()
    }
}
