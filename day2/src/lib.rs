use std::{error::Error, fs};

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

fn get_shape_score(my_shape: &Shape) -> Result<u32, &'static str> {
    match my_shape {
        Shape::Rock => Ok(1),
        Shape::Paper => Ok(2),
        Shape::Scissors => Ok(3),
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
        Ok(0)
    } else if draw_scenario {
        Ok(3)
    } else if lose_scenario {
        Ok(6)
    } else {
        Err("Missing scenario")
    }
}

pub fn get_round_score(round_shapes: &str) -> Result<u32, &'static str> {
    // TODO Parse round
    if let Ok(parsed_shapes) = parse_round(round_shapes) {
        if let Ok(scores) = get_total_score(parsed_shapes) {
            Ok(scores.2)
        } else {
            Err("Something went wrong getting total score")
        }
    } else {
        eprint!("Bad shapes: {round_shapes}");
        Err("Something went wrong parsing round")
    }
}

fn parse_round(round_shapes: &str) -> Result<(Shape, Shape), &'static str> {
    let (opponent, mine) = round_shapes.split_at(1);
    Ok((Shape::new(opponent)?, Shape::new(mine)?))
}

fn get_total_score(parsed_shapes: (Shape, Shape)) -> Result<(u32, u32, u32), &'static str> {
    let shape_score = get_shape_score(&parsed_shapes.1)?;
    let outcome_score = get_outcome_score(&parsed_shapes)?;
    Ok((shape_score, outcome_score, shape_score + outcome_score))
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut total_score = 0;
    if let Ok(contents) = fs::read_to_string("input.txt") {
        for line in contents.lines() {
            total_score += get_round_score(line)?;
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
        if let Ok(total_score) = get_round_score(line) {
            assert_eq!(total_score, 8);
        }
    }

    #[test]
    fn test_parse_round() {
        let line = "A Y";
        if let Ok(parsed_shapes) = parse_round(line) {
            assert_eq!(parsed_shapes.0, Shape::Rock);
            assert_eq!(parsed_shapes.1, Shape::Paper);
        }
        let line = "B X";
        if let Ok(parsed_shapes) = parse_round(line) {
            assert_eq!(parsed_shapes.0, Shape::Paper);
            assert_eq!(parsed_shapes.1, Shape::Rock);
        }
        let line = "C X";
        if let Ok(parsed_shapes) = parse_round(line) {
            dbg!("{line}");
            assert_eq!(parsed_shapes.0, Shape::Scissors);
            assert_eq!(parsed_shapes.1, Shape::Scissors);
        }
    }
}
