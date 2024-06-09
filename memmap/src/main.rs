use std::fs::File;
// use std::io::Read;

use memmap2::Mmap;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("/usr/share/dict/words")?;

    let mmap = unsafe { Mmap::map(&file)? };
    println!("{}", &mmap[..].len());

    Ok(())
}
