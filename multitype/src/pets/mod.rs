mod cat;
pub use cat::Cat;

mod dog;
pub use dog::Dog;

pub trait Speak {
    fn speak(&self);
}
