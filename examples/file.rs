use std::fs::{File};
use std::io;
use std::io::Read;

fn main() -> Result<(), io::Error> {
    let mut s = String::new();
    File::open("sample.txt")?
        .read_to_string(&mut s)?;
    println!("{}", s);
    Ok(())
}