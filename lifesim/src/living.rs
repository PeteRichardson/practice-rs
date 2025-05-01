pub trait Living {
    fn name(&self) -> &str;
    fn live(&mut self);
    fn die(&self) {
        println!("{} has died!", self.name());
    }
}
