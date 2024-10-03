use std::{error::Error, fs};

pub fn split_string(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn find_matching_characters<'a>(string1: &'a str, string2: &'a str) -> Vec<char> {
    let mut matching_chars = vec![];
    for char1 in string1.chars() {
        for char2 in string2.chars() {
            if char1 == char2 {
                matching_chars.push(char1.to_owned())
            };
        }
    }
    matching_chars
}

pub fn run() -> Result<(), Box<dyn Error>> {
    if let Ok(contents) = fs::read_to_string("input.txt") {
        for line in contents.lines() {
            let (compartment_1, compartment_2) = split_string(line);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CASES: [(&str, &str); 6] = [
        ("vJrwpWtwJgWrhcsFMMfFFhFp", "p"),
        ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "L"),
        ("PmmdzqPrVvPwwTWBwg", "P"),
        ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "v"),
        ("ttgJtRGJQctTZtZT", "t"),
        ("CrZsJsPPZsGzwwsLwLmpwMDw", "s"),
    ];
    fn test_parse_row() {
        let test_case = ("vJrwpWtwJgWrhcsFMMfFFhFp", "p");
    }

    #[test]
    fn test_split_string() {
        let test_case = ("vJrwpWtwJgWrhcsFMMfFFhFp", "p");
        let (compartment_1, compartment_2) = split_string(test_case.0);
        assert_eq!(compartment_1, "vJrwpWtwJgWr");
        assert_eq!(compartment_2, "hcsFMMfFFhFp");

        let test_case = ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "p");
        let (compartment_1, compartment_2) = split_string(test_case.0);
        assert_eq!(compartment_1, "jqHRNqRjqzjGDLGL");
        assert_eq!(compartment_2, "rsFMfFZSrLrFZsSL");

        let test_case = ("PmmdzqPrVvPwwTWBwg", "p");
        let (compartment_1, compartment_2) = split_string(test_case.0);
        assert_eq!(compartment_1, "PmmdzqPrV");
        assert_eq!(compartment_2, "vPwwTWBwg");
    }

    #[test]
    fn test_find_matching_characters() {
        let test_case = ("vJrwpWtwJgWrhcsFMMfFFhFp", "p");
        let (compartment_1, compartment_2) = split_string(test_case.0);
        let matching_chars = find_matching_characters(compartment_1, compartment_2);
        assert_eq!(
            matching_chars[0],
            test_case.1.chars().next().expect("empty string")
        );

        let test_case = ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "L");
        let (compartment_1, compartment_2) = split_string(test_case.0);
        let matching_chars = find_matching_characters(compartment_1, compartment_2);
        assert_eq!(
            matching_chars[0],
            test_case.1.chars().next().expect("empty string")
        );

        let test_case = ("PmmdzqPrVvPwwTWBwg", "P");
        let (compartment_1, compartment_2) = split_string(test_case.0);
        let matching_chars = find_matching_characters(compartment_1, compartment_2);
        assert_eq!(
            matching_chars[0],
            test_case.1.chars().next().expect("empty string")
        );
    }
}
