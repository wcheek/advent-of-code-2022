use std::{collections::HashMap, error::Error, fs};

fn find_matching_characters<'a>(string1: &'a str, string2: &'a str) -> Vec<String> {
    let mut matching_chars = vec![];
    for char1 in string1.chars() {
        for char2 in string2.chars() {
            if char1 == char2 {
                matching_chars.push(char1.to_string().to_owned())
            };
        }
    }
    matching_chars
}

fn split_string(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn get_scores(chars: Vec<String>) -> Vec<usize> {
    const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut alphabet_hash = HashMap::new();
    for (ind, char) in ALPHABET.chars().enumerate() {
        alphabet_hash.insert(char, ind + 1);
    }
    let mut scores = vec![];
    for matched_chars in chars.iter() {
        scores.push(alphabet_hash[&matched_chars.chars().next().expect("string")]);
    }
    scores
}

pub fn run() -> Result<(), Box<dyn Error>> {
    if let Ok(contents) = fs::read_to_string("input.txt") {
        let mut total_score = 0;
        for line in contents.lines() {
            let (compartment_1, compartment_2) = split_string(line);
            let matching_chars = find_matching_characters(compartment_1, compartment_2);
            let scores = get_scores(matching_chars);
            total_score += scores[0];
        }
        println!("{total_score}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const _TEST_CASES: [(&str, &str); 6] = [
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
        assert_eq!(matching_chars[0], test_case.1);

        let test_case = ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "L");
        let (compartment_1, compartment_2) = split_string(test_case.0);
        let matching_chars = find_matching_characters(compartment_1, compartment_2);
        assert_eq!(matching_chars[0], test_case.1);

        let test_case = ("PmmdzqPrVvPwwTWBwg", "P");
        let (compartment_1, compartment_2) = split_string(test_case.0);
        let matching_chars = find_matching_characters(compartment_1, compartment_2);
        assert_eq!(matching_chars[0], test_case.1);
    }

    #[test]
    fn test_get_scores() {
        let test_case = vec![String::from("p"), String::from("L"), String::from("P")];
        let scores = get_scores(test_case);
        assert_eq!(scores[0], 16);
        assert_eq!(scores[1], 38);
        assert_eq!(scores[2], 42);
    }
}
