// Running through some examples from chapter 4 of
// The Rust Programming Language, 2nd ed.

fn append_s(s: &mut str) -> String {
    s.to_owned()  + "s"
}

fn main() {
    let s0 : String;
    {
        let mut s1 = String::from("Hello");
        s1 = append_s(&mut s1);
        s0 = s1;
        //println!("s1: {}", s1);
    }
    println!("s0: {}", s0);
}
