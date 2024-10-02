use std::{error::Error, fs};

#[derive(Debug)]
pub struct Calories {
    pub total_calories: u32,
}

fn get_total_calories_vec(contents: String) -> Result<Vec<Calories>, Box<dyn Error>> {
    let mut results: Vec<Calories> = vec![];
    let mut total_calories = 0;
    for line in contents.lines() {
        if let Ok(value) = line.parse::<u32>() {
            total_calories += value;
        } else {
            results.push(Calories { total_calories });
            total_calories = 0
        };
    }
    Ok(results)
}

pub fn run() -> Result<(), Box<dyn Error>> {
    if let Ok(contents) = fs::read_to_string("input.txt") {
        if let Ok(mut total_calories) = get_total_calories_vec(contents) {
            let mut max_calories = 0;
            total_calories.sort_by(|a, b| b.total_calories.cmp(&a.total_calories));
            for calorie in &total_calories {
                if calorie.total_calories > max_calories {
                    max_calories = calorie.total_calories;
                }
            }
            println!(
                "{}",
                &total_calories[0..3]
                    .iter()
                    .map(|calories| calories.total_calories)
                    .sum::<u32>()
            )
        }
    }
    Ok(())
}
