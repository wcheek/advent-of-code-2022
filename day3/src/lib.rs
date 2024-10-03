use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse_row_case() -> Result<(), String> {
        let test_cases = [
            ("vJrwpWtwJgWrhcsFMMfFFhFp", "p"),
            ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "L"),
            ("PmmdzqPrVvPwwTWBwg", "P"),
            ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "v"),
            ("ttgJtRGJQctTZtZT", "t"),
            ("CrZsJsPPZsGzwwsLwLmpwMDw", "s"),
        ]
        .iter()
        .try_for_each(|(row, expected)| Ok(()))?;
        Ok(())
    }
}
