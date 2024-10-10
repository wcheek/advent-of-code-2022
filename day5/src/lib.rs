use regex::Regex;
use std::{error::Error, fs};

mod stack;

fn parse_layout(initial_layout_str: &str) -> Vec<stack::Stack<&str>> {
    let mut empty_layout: Vec<stack::Stack<&str>> = vec![stack::Stack::new(); 9];
    let re = Regex::new(r"(\[\w\]|\s{3})\s(\[\w\]|\s{3})\s(\[\w\]|\s{3})\s(\[\w\]|\s{3})\s(\[\w\]|\s{3})\s(\[\w\]|\s{3})\s(\[\w\]|\s{3})\s(\[\w\]|\s{3})\s(\[\w\]|\s{3})").unwrap();

    let lines = initial_layout_str.lines();
    for line in lines {
        if let Some(caps) = re.captures(line) {
            let caps_enum = caps.iter().enumerate();
            for (ind, cap) in caps_enum {
                if let Some(cap) = cap {
                    if cap.as_str() != "   " {
                        let val = cap.as_str().trim_start_matches("[").trim_end_matches("]");
                        if ind > 0 {
                            empty_layout[ind - 1].push(val);
                        }
                    };
                }
            }
        };
    }
    empty_layout
}

fn reverse_layout(
    initial_layout: &mut [stack::Stack<&'static str>],
) -> Vec<stack::Stack<&'static str>> {
    initial_layout
        .iter_mut()
        .map(|stack| {
            stack.reverse();
            stack.clone()
        })
        .collect()
}

pub fn run() -> Result<(), Box<dyn Error>> {
    if let Ok(contents) = fs::read_to_string("input.txt") {
        let (initial_layout, instructions) = contents.split_once("\r\n\r").unwrap();
        parse_layout(initial_layout);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_initial_layout() {
        let initial_layout = "[V]         [T]         [J]        \r\n[Q]         [M] [P]     [Q]     [J]\r\n[W] [B]     [N] [Q]     [C]     [T]\r\n[M] [C]     [F] [N]     [G] [W] [G]\r\n[B] [W] [J] [H] [L]     [R] [B] [C]\r\n[N] [R] [R] [W] [W] [W] [D] [N] [F]\r\n[Z] [Z] [Q] [S] [F] [P] [B] [Q] [L]\r\n[C] [H] [F] [Z] [G] [L] [V] [Z] [H]\r\n 1   2   3   4   5   6   7   8   9 ";
        let expected = vec![
            stack::Stack::build(vec!["V", "Q", "W", "M", "B", "N", "Z", "C"]),
            stack::Stack::build(vec!["B", "C", "W", "R", "Z", "H"]),
            stack::Stack::build(vec!["J", "R", "Q", "F"]),
            stack::Stack::build(vec!["T", "M", "N", "F", "H", "W", "S", "Z"]),
            stack::Stack::build(vec!["P", "Q", "N", "L", "W", "F", "G"]),
            stack::Stack::build(vec!["W", "P", "L"]),
            stack::Stack::build(vec!["J", "Q", "C", "G", "R", "D", "B", "V"]),
            stack::Stack::build(vec!["W", "B", "N", "Q", "Z"]),
            stack::Stack::build(vec!["J", "T", "G", "C", "F", "L", "H"]),
        ];
        assert_eq!(expected, parse_layout(initial_layout));
    }

    #[test]
    fn test_reverse_layout() {
        let initial_layout_str = "[V]         [T]         [J]        \r\n[Q]         [M] [P]     [Q]     [J]\r\n[W] [B]     [N] [Q]     [C]     [T]\r\n[M] [C]     [F] [N]     [G] [W] [G]\r\n[B] [W] [J] [H] [L]     [R] [B] [C]\r\n[N] [R] [R] [W] [W] [W] [D] [N] [F]\r\n[Z] [Z] [Q] [S] [F] [P] [B] [Q] [L]\r\n[C] [H] [F] [Z] [G] [L] [V] [Z] [H]\r\n 1   2   3   4   5   6   7   8   9 ";
        let mut initial_layout = parse_layout(initial_layout_str);

        let reversed_layout = reverse_layout(&mut initial_layout);
        let expected = vec![
            stack::Stack::build(vec!["C", "Z", "N", "B", "M", "W", "Q", "V"]),
            stack::Stack::build(vec!["H", "Z", "R", "W", "C", "B"]),
            stack::Stack::build(vec!["F", "Q", "R", "J"]),
            stack::Stack::build(vec!["Z", "S", "W", "H", "F", "N", "M", "T"]),
            stack::Stack::build(vec!["G", "F", "W", "L", "N", "Q", "P"]),
            stack::Stack::build(vec!["L", "P", "W"]),
            stack::Stack::build(vec!["V", "B", "D", "R", "G", "C", "Q", "J"]),
            stack::Stack::build(vec!["Z", "Q", "N", "B", "W"]),
            stack::Stack::build(vec!["H", "L", "F", "C", "G", "T", "J"]),
        ];

        assert_eq!(expected, reversed_layout);
    }
}
