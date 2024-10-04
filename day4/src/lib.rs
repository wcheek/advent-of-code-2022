use std::cmp::Ordering;
use std::{error::Error, fs};

fn get_sections(line: &str) -> (Vec<u32>, Vec<u32>) {
    let sec_vec = line.split(',').collect::<Vec<&str>>();
    let mut sec1_nums = sec_vec[0]
        .split('-')
        .map(|num_str| num_str.parse::<u32>().unwrap());
    let sec1_num1 = sec1_nums.next().unwrap();
    let sec1_num2 = sec1_nums.next().unwrap();
    let mut sec2_nums = sec_vec[1]
        .split('-')
        .map(|num_str| num_str.parse::<u32>().unwrap());
    let sec2_num1 = sec2_nums.next().unwrap();
    let sec2_num2 = sec2_nums.next().unwrap();

    let sec1_seq = Vec::from_iter(sec1_num1..sec1_num2 + 1);
    let sec2_seq = Vec::from_iter(sec2_num1..sec2_num2 + 1);

    (sec1_seq, sec2_seq)
}

fn check_for_full_overlap(elf1: Vec<u32>, elf2: Vec<u32>) -> bool {
    match elf1.len().cmp(&elf2.len()) {
        Ordering::Greater => elf1[0] <= elf2[0] && elf1.last() >= elf2.last(),
        Ordering::Less => elf2[0] <= elf1[0] && elf2.last() >= elf1.last(),
        Ordering::Equal => elf2[0] == elf1[0] && elf2.last() == elf1.last(),
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    if let Ok(contents) = fs::read_to_string("input.txt") {
        let mut result = 0;
        let lines = contents.lines();
        for line in lines {
            let (elf1, elf2) = get_sections(line);
            if check_for_full_overlap(elf1, elf2) {
                result += 1;
            }
        }
        println!("{result}");
    }
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
        assert_eq!(sec1, [2, 3]);
        assert_eq!(sec2, [4, 5]);

        let case = "5-7,7-9";
        let (sec1, sec2) = get_sections(case);
        assert_eq!(sec1, [5, 6, 7]);
        assert_eq!(sec2, [7, 8, 9]);
    }

    #[test]
    fn test_check_for_overlap() {
        let case1 = vec![2, 3, 4, 5, 6, 7, 8];
        let case2 = vec![3, 4, 5, 6, 7];
        assert!(check_for_full_overlap(case1, case2));

        let case1 = vec![6];
        let case2 = vec![4, 5, 6];
        assert!(check_for_full_overlap(case1, case2));
    }
}
