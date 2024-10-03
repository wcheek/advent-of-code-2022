use std::{error::Error, fs, process};

#[derive(PartialEq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn new(shape_code: &str) -> Result<Shape, &'static str> {
        match shape_code {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),
            _ => Err("Shape code does not correspond to a shape"),
        }
    }
}

fn get_shape_score(my_shape: &Shape) -> u32 {
    match my_shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn get_outcome_score(parsed_shapes: &(Shape, Shape)) -> Result<u32, &'static str> {
    // Winning scenarios
    let rock_wins_scissors = parsed_shapes.0 == Shape::Scissors && parsed_shapes.1 == Shape::Rock;
    let paper_wins_rock = parsed_shapes.0 == Shape::Rock && parsed_shapes.1 == Shape::Paper;
    let scissors_wins_paper = parsed_shapes.0 == Shape::Paper && parsed_shapes.1 == Shape::Scissors;
    let win_scenario = paper_wins_rock || rock_wins_scissors || scissors_wins_paper;

    // Draw scenarios
    let rock_rock = parsed_shapes.0 == Shape::Rock && parsed_shapes.1 == Shape::Rock;
    let paper_paper = parsed_shapes.0 == Shape::Paper && parsed_shapes.1 == Shape::Paper;
    let scissors_scissors =
        parsed_shapes.0 == Shape::Scissors && parsed_shapes.1 == Shape::Scissors;
    let draw_scenario = rock_rock || paper_paper || scissors_scissors;

    // Lose scenarios
    let rock_loses_scissors = parsed_shapes.0 == Shape::Rock && parsed_shapes.1 == Shape::Scissors;
    let paper_loses_rock = parsed_shapes.0 == Shape::Paper && parsed_shapes.1 == Shape::Rock;
    let scissors_loses_paper =
        parsed_shapes.0 == Shape::Scissors && parsed_shapes.1 == Shape::Paper;
    let lose_scenario = paper_loses_rock || rock_loses_scissors || scissors_loses_paper;

    if win_scenario {
        Ok(6)
    } else if draw_scenario {
        Ok(3)
    } else if lose_scenario {
        Ok(0)
    } else {
        Err("Missing scenario")
    }
}

pub fn get_round_score(round_shapes: &str) -> u32 {
    let parsed_shapes = parse_round(round_shapes);
    get_total_score(parsed_shapes).2
}

fn parse_round(round_shapes: &str) -> (Shape, Shape) {
    let mut split_strings = round_shapes.split_whitespace();
    let opponent = split_strings.next().unwrap();
    let mine = split_strings.next().unwrap();

    let opponent_shape = Shape::new(opponent).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments: {err} {opponent}");
        process::exit(1);
    });
    let my_shape = Shape::new(mine).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments: {err} {mine}");
        process::exit(1);
    });
    (opponent_shape, my_shape)
}

fn get_total_score(parsed_shapes: (Shape, Shape)) -> (u32, u32, u32) {
    let shape_score = get_shape_score(&parsed_shapes.1);
    let outcome_score = get_outcome_score(&parsed_shapes).unwrap_or_else(|err| {
        eprintln!("Problem getting the outcome score: {err}");
        process::exit(1);
    });
    (shape_score, outcome_score, shape_score + outcome_score)
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut total_score = 0;
    if let Ok(contents) = fs::read_to_string("input.txt") {
        for line in contents.lines() {
            total_score += get_round_score(line);
        }
    }
    println!("{total_score}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_round_score() {
        let line = "A Y";
        let total_score = get_round_score(line);
        assert_eq!(total_score, 8);
    }

    #[test]
    fn test_parse_round() {
        let line = "A Y";
        let parsed_shapes = parse_round(line);
        assert_eq!(parsed_shapes.0, Shape::Rock);
        assert_eq!(parsed_shapes.1, Shape::Paper);

        let line = "B X";
        let parsed_shapes = parse_round(line);
        assert_eq!(parsed_shapes.0, Shape::Paper);
        assert_eq!(parsed_shapes.1, Shape::Rock);

        let line = "C X";
        let parsed_shapes = parse_round(line);
        assert_eq!(parsed_shapes.0, Shape::Scissors);
        assert_eq!(parsed_shapes.1, Shape::Rock);
    }
}
