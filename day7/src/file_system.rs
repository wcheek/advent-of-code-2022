use std::collections::HashMap;

pub struct MyFile {
    pub name: String,
    pub size: u8,
}
pub struct MyDir {
    pub name: String,
    pub files: Vec<MyFile>,
}

pub struct MyFS<'a> {
    pub cwd: String,
    pub cwd_dirs: HashMap<&'a str, MyDir>,
}

impl<'a> MyFS<'a> {
    pub fn new() -> Self {
        MyFS {
            cwd: String::new(),
            cwd_dirs: HashMap::new(),
        }
    }
}

impl MyDir {
    pub fn new() -> Self {
        MyDir {
            name: String::new(),
            files: vec![],
        }
    }
}
