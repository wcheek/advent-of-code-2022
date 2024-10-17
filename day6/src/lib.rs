use std::{error::Error, fs};

pub fn run() -> Result<(), Box<dyn Error>> {
    if let Ok(contents) = fs::read_to_string("input.txt") {
        dbg!(contents);
    } else {
        panic!("Contents could not be read");
    }
    Ok(())
}
