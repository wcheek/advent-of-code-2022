use std::{collections::HashSet, error::Error, fs, hash::Hash};

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

pub fn run() -> Result<(), Box<dyn Error>> {
    if let Ok(contents) = fs::read_to_string("input.txt") {
        let mut current = ['0'; 4];
        for (ind, c) in contents.char_indices() {
            current[ind % 4] = c;
            if has_unique_elements(current) && !current.contains(&'0') {
                dbg!(&current);
                dbg!(&ind + 1);
                break;
            }
        }
    } else {
        panic!("Contents could not be read");
    }
    Ok(())
}
