use std::{error::Error, fs};
mod file_system;
use file_system::{MyDir, MyFS};

fn parse_directory<'a>(my_fs: &'a mut MyFS<'a>, dir_name: &'a str) {
    my_fs.cwd_dirs.insert(dir_name, MyDir::new());
}

fn handle_cd(my_fs: &mut MyFS, to: String) {
    match to.as_str() {
        "/" => my_fs.cwd = String::from("/"),
        _ => todo!("bla"),
    }
}

fn handle_command(my_fs: &mut MyFS, split_strings: &[&str]) {
    match split_strings[0] {
        "cd" => handle_cd(my_fs, split_strings[1].to_string()),
        _ => todo!("bla"),
    }
    dbg!(split_strings);
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut my_fs = MyFS::new();
    if let Ok(contents) = fs::read_to_string("input.txt") {
        let lines = contents.lines();
        for line in lines {
            let split_words: Vec<&str> = line.split(' ').collect();
            match split_words[0] {
                "$" => handle_command(&mut my_fs, &split_words[1..split_words.len()]),
                "dir" => parse_directory(&mut my_fs, &split_words[1]),
                _ => todo!("bla"),
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

    #[test]
    fn test_handle_cd() {
        let mut my_fs = MyFS::new();
        handle_cd(&mut my_fs, String::from("/"));
        assert_eq!(my_fs.cwd, String::from("/"));
    }

    #[test]
    fn test_parse_dirs() {}
}
