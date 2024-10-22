use std::{error::Error, fs, str::Split};

fn process_command(split_strings: &[&str]) {}

pub fn run() -> Result<(), Box<dyn Error>> {
    if let Ok(contents) = fs::read_to_string("input.txt") {
        let split_words: Vec<&str> = contents.split(' ').collect();
        if split_words[0] == "$" {
            process_command(&split_words[1..split_words.len() - 1]);
        }
    } else {
        panic!("Contents could not be read");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_command() {
        let cd = "$ cd /";
    }
}
