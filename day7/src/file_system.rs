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
    pub catalog: HashMap<&'a str, MyDir>,
}

impl MyFS<'_> {
    pub fn new() -> Self {
        MyFS {
            cwd: String::new(),
            catalog: HashMap::new(),
        }
    }
}
