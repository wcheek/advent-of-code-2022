use std::{collections::HashMap, error::Error, fs};

fn find_matching_characters<'a>(string1: &'a str, string2: &'a str) -> Vec<String> {
    let mut matching_chars = vec![];
    for char1 in string1.chars() {
        for char2 in string2.chars() {
            if char1 == char2 && !matching_chars.contains(&char1.to_string()) {
                matching_chars.push(char1.to_string().to_owned())
            };
        }
    }
    matching_chars
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
        let mut lines = contents.lines();
        while let Some(elf1) = lines.next() {
            let elf2 = lines.next().expect("Should not panic");
            let elf3 = lines.next().expect("Should not panic");
            let matching_chars = find_matching_characters(elf1, elf2);
            let matching_chars = find_matching_characters(&matching_chars.join(""), elf3);
            total_score += get_scores(matching_chars)[0];
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

    #[test]
    fn test_find_matching_characters() {
        let test_case1 = ("vJrwpWtwJgWrhcsFMMfFFhFp", "p");
        let test_case2 = ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "L");
        let test_case3 = ("PmmdzqPrVvPwwTWBwg", "p");
        let matching_chars = find_matching_characters(test_case1.0, test_case2.0);
        let matching_chars = find_matching_characters(&matching_chars.join(""), test_case3.0);
        println!("{:?}", matching_chars.join(""));
        // assert_eq!(matching_chars[0],);

        // let test_case = ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "L");
        // let (compartment_1, compartment_2) = split_string(test_case.0);
        // let matching_chars = find_matching_characters(compartment_1, compartment_2);
        // assert_eq!(matching_chars[0], test_case.1);
        //
        // let test_case = ("PmmdzqPrVvPwwTWBwg", "P");
        // let (compartment_1, compartment_2) = split_string(test_case.0);
        // let matching_chars = find_matching_characters(compartment_1, compartment_2);
        // assert_eq!(matching_chars[0], test_case.1);
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
