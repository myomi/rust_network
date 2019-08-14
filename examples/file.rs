use std::fs::{File};
use std::io;
use std::io::Read;

fn main() -> Result<(), io::Error> {
    let f = File::open("sample.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => {
            println!("{}", s);
            Ok(())
        },
        Err(e) => Err(e),
    }
}