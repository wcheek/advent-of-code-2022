use std::{error::Error, fs};

pub fn run() -> Result<(), Box<dyn Error>> {
    if let Ok(contents) = fs::read_to_string("input.txt") {
        dbg!(contents.split_once("\r\n\r"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_initial_layout() {}

    #[test]
    fn test_parse_instructions() {}
}
