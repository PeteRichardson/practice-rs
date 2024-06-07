use log::{debug, info};
use std::thread::{self, ThreadId};
fn enumerated_squares(x: u32) -> (ThreadId, u32) {
    let id = thread::current().id();
    info!("Hello from thread id {:?}.   {}^2 = {}", id, x, x * x);
    (id, x * x)
}

fn main() {
    env_logger::init();
    let a = 5;
    let b = 7;
    let t1 = thread::spawn(move || enumerated_squares(a));
    let t2 = thread::spawn(move || enumerated_squares(b));

    debug!("Hello from the main thread!");

    let (i1, r1) = t1.join().unwrap();
    let (i2, r2) = t2.join().unwrap();

    debug!("ids = {:?}, {:?}", i1, i2);
    info!("squares = {}, {}", r1, r2);
    println!("Sum of squares = {}", r1 + r2);
}
