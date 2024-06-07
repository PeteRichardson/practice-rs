use std::thread;

fn f() {
    println!("Hello from thread id {:?}", thread::current().id());
}

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from the main thread!");

    t1.join().unwrap();
    t2.join().unwrap();
}
