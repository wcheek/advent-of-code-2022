use regex::Regex;
use std::{error::Error, fs, str};

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
                    if cap.as_str() != "   " && ind > 0 {
                        let val = cap.as_str().trim_start_matches("[").trim_end_matches("]");
                        empty_layout[ind - 1].push(val);
                    };
                }
            }
        };
    }
    empty_layout
}

fn reverse_layout<'a>(initial_layout: &mut [stack::Stack<&'a str>]) -> Vec<stack::Stack<&'a str>> {
    initial_layout
        .iter_mut()
        .map(|stack| {
            stack.reverse();
            stack.clone()
        })
        .collect()
}

fn perform_instruction<'a>(
    mut layout: Vec<stack::Stack<&'a str>>,
    instruction: &str,
) -> Vec<stack::Stack<&'a str>> {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    if let Some(caps) = re.captures(instruction) {
        let init_pos = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let move_pos = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

        let mut num_to_move = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        while num_to_move > 0 {
            if let Some(pop_box) = layout[init_pos - 1].pop() {
                layout[move_pos - 1].push(pop_box);
                num_to_move -= 1;
            };
        }
    };
    layout
}

fn get_answer(layout: Vec<stack::Stack<&str>>) -> String {
    layout
        .iter()
        .map(|stk| *stk.peek().unwrap())
        .collect::<Vec<&str>>()
        .join("")
}

pub fn run() -> Result<(), Box<dyn Error>> {
    if let Ok(contents) = fs::read_to_string("input.txt") {
        if let Some(split) = contents.split_once("\r\n\r") {
            let (initial_layout_str, instructions) = split;

            let mut layout = parse_layout(initial_layout_str);
            layout = reverse_layout(&mut layout);

            for instruction in instructions.lines() {
                layout = perform_instruction(layout, instruction.trim());
            }
            dbg!(get_answer(layout));
        };
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

        assert_eq!(expected, reverse_layout(&mut initial_layout));
    }

    #[test]
    fn test_perform_instruction() {
        let instruction = "move 2 from 1 to 7";
        let orig_layout = vec![
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
        let new_layout = vec![
            stack::Stack::build(vec!["C", "Z", "N", "B", "M", "W"]),
            stack::Stack::build(vec!["H", "Z", "R", "W", "C", "B"]),
            stack::Stack::build(vec!["F", "Q", "R", "J"]),
            stack::Stack::build(vec!["Z", "S", "W", "H", "F", "N", "M", "T"]),
            stack::Stack::build(vec!["G", "F", "W", "L", "N", "Q", "P"]),
            stack::Stack::build(vec!["L", "P", "W"]),
            stack::Stack::build(vec!["V", "B", "D", "R", "G", "C", "Q", "J", "V", "Q"]),
            stack::Stack::build(vec!["Z", "Q", "N", "B", "W"]),
            stack::Stack::build(vec!["H", "L", "F", "C", "G", "T", "J"]),
        ];
        assert_eq!(new_layout, perform_instruction(orig_layout, instruction));
    }
}
