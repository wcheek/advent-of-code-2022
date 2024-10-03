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
    fn get_shape_score(&self) -> u32 {
        match &self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(PartialEq, Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn new(desired_outcome_str: &str) -> Result<Outcome, &'static str> {
        match desired_outcome_str {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err("Outcome code does not correspond to an outcome"),
        }
    }
    fn get_score(&self) -> u32 {
        match &self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
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

fn get_correct_shape(opponent_shape: &Shape, desired_outcome: &Outcome) -> Shape {
    match opponent_shape {
        Shape::Rock => match desired_outcome {
            Outcome::Win => Shape::Paper,
            Outcome::Draw => Shape::Rock,
            Outcome::Lose => Shape::Scissors,
        },
        Shape::Paper => match desired_outcome {
            Outcome::Win => Shape::Scissors,
            Outcome::Draw => Shape::Paper,
            Outcome::Lose => Shape::Rock,
        },
        Shape::Scissors => match desired_outcome {
            Outcome::Win => Shape::Rock,
            Outcome::Draw => Shape::Scissors,
            Outcome::Lose => Shape::Paper,
        },
    }
}

fn parse_round(round_shapes: &str) -> (Shape, Shape, Outcome) {
    let mut split_strings = round_shapes.split_whitespace();
    let opponent_str = split_strings.next().unwrap();
    let desired_outcome_str = split_strings.next().unwrap();

    let opponent_shape = Shape::new(opponent_str).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments: {err} {opponent_str}");
        process::exit(1);
    });

    let desired_outcome = Outcome::new(desired_outcome_str).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments: {err} {desired_outcome_str}");
        process::exit(1);
    });
    let my_shape = get_correct_shape(&opponent_shape, &desired_outcome);
    (opponent_shape, my_shape, desired_outcome)
}

fn get_total_score(parsed_shapes: (Shape, Shape, Outcome)) -> (u32, u32, u32) {
    let shape_score = parsed_shapes.1.get_shape_score();
    let outcome_score = parsed_shapes.2.get_score();
    (shape_score, outcome_score, shape_score + outcome_score)
}

fn get_round_score(round_shapes: &str) -> u32 {
    let parsed_round = parse_round(round_shapes);
    get_total_score(parsed_round).2
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
        assert_eq!(total_score, 4);
    }

    #[test]
    fn test_parse_round() {
        let line = "A Y";
        let parsed_shapes = parse_round(line);
        assert_eq!(parsed_shapes.0, Shape::Rock);
        assert_eq!(parsed_shapes.2, Outcome::Draw);

        let line = "B X";
        let parsed_shapes = parse_round(line);
        assert_eq!(parsed_shapes.0, Shape::Paper);
        assert_eq!(parsed_shapes.2, Outcome::Lose);

        let line = "C Z";
        let parsed_shapes = parse_round(line);
        assert_eq!(parsed_shapes.0, Shape::Scissors);
        assert_eq!(parsed_shapes.2, Outcome::Win);
    }
}
