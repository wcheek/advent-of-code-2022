use std::{error::Error, fs};

fn get_sections(line: &str) -> (&[u32], &[u32]) {
    let sec_vec = line.split(",").collect::<Vec<&str>>();
    let mut sec1_nums = sec_vec[0]
        .split("-")
        .map(|num_str| num_str.parse::<u32>().unwrap());
    let sec1_num1 = sec1_nums.next().unwrap();
    let sec1_num2 = sec1_nums.next().unwrap();
    let mut sec2_nums = sec_vec[1]
        .split("-")
        .map(|num_str| num_str.parse::<u32>().unwrap());
    let sec2_num1 = sec2_nums.next().unwrap();
    let sec2_num2 = sec2_nums.next().unwrap();

    (
        Vec::from_iter(sec1_num1..sec1_num2).as_slice(),
        Vec::from_iter(sec2_num1.clone()..sec2_num2.clone()).as_slice(),
    )
}
pub fn run() -> Result<(), Box<dyn Error>> {
    if let Ok(contents) = fs::read_to_string("input.txt") {}
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sections() {
        let case = "2-4,6-8";
        let (sec1, sec2) = get_sections(case);
        assert_eq!(sec1, [2, 3, 4]);
        assert_eq!(sec2, [6, 7, 8]);

        let case = "2-3,4-5";
        let (sec1, sec2) = get_sections(case);
        assert_eq!(sec1, [2, 3, 4]);
        assert_eq!(sec2, [4, 5]);

        let case = "5-7,7-9";
        let (sec1, sec2) = get_sections(case);
        assert_eq!(sec1, [5, 6, 7]);
        assert_eq!(sec2, [7, 8, 9]);
    }
}
