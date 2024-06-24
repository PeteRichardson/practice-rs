// Running through some examples from chapter 4 of
// The Rust Programming Language, 2nd ed.

fn append_s(s: &mut String) { 
    s.push('s')
}

fn main() {
    let mut s0 = String::from("Hello"); // s0's scope begins. It's a mutable string
    append_s(&mut s0);
    println!("s0: {}", s0);
}
