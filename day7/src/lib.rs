use std::{error::Error, fs};
mod file_system;
use file_system::MyFS;

fn handle_cd(to: String) {
    if to == "/" {}
}

fn handle_command(my_fs: &MyFS, split_strings: &[&str]) {
    match split_strings[0] {
        "cd" => handle_cd(split_strings[1].to_string()),
        _ => panic!("bla"),
    }
    dbg!(split_strings);
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let my_fs = MyFS::new();
    if let Ok(contents) = fs::read_to_string("input.txt") {
        let lines = contents.lines();
        for line in lines {
            let split_words: Vec<&str> = line.split(' ').collect();
            if split_words[0] == "$" {
                handle_command(&my_fs, &split_words[1..split_words.len()]);
            }
        }
    } else {
        panic!("Contents could not be read");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use file_system::MyFS;

    use super::*;

    #[test]
    fn test_process_command() {
        let cd = "$ cd /";
        let my_fs = MyFS::new();
    }
}
