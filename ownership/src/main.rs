// Running through some examples from chapter 4 of
// The Rust Programming Language, 2nd ed.

fn append_s(s: &mut String) {
    s.push('s')
}

fn main() {
    let mut strings = [
        "Pete".to_string(),
        "Wendy".to_string(),
        "Kat".to_string(),
        "Bella".to_string(),
    ];
    strings.iter_mut().for_each(|s| {
        append_s(s);
        println!("s: {}", s)
    });
}
