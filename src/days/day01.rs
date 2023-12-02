use std::fs::read_to_string;
use std::collections::HashMap;

pub fn solve(input_file: String) -> (String, String) {

    let mut result01 = 0;
    let mut result02 = 0;

    let digits = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let mut numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    numbers.extend(&digits);

    // This is a let binding to avoid weird lifetime issues
    let data = read_to_string(input_file).unwrap();
    let input = data.lines();
    for line in input {

        let mut value = calc_line(line, &digits);
        result01 += value;

        value = calc_line(line, &numbers);
        result02 += value;

    }

    (format!("{result01}").to_string(), format!("{result02}").to_string())
}

// Find both left and right
fn find_target(input: &str, target: &str) -> (isize, isize) {
    let mut from_left: isize = -1;
    let mut from_right: isize = -1;
    match input.find(&target) {
        Some(result) => from_left = result as isize,
        None => (),
    }
    match input.rfind(&target) {
        Some(result) => from_right = result as isize,
        None => (),
    }
    (from_left, from_right)
}

fn calc_line(input: &str, targets: &HashMap<&str, usize>) -> usize {

    let mut ones: usize = 0;
    let mut ones_place = -1;
    let mut tens: usize = 0;
    let mut tens_place = 999_999_999;

    for (target, value) in targets {
        let (left, right) = find_target(input, target);
        if left != -1 && left < tens_place {
            tens_place = left;
            tens = *value;
        }
        if right != -1 && right > ones_place {
            ones_place = right;
            ones = *value;
        }
    }

    format!("{}{}", tens, ones).to_string().parse().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_01 () {
        let (result1, _result2) = solve("input/test01_01.txt".to_string());
        assert_eq!(result1, "142");
    }

    #[test]
    fn input_02 () {
        let (_result1, result2) = solve("input/test02_01.txt".to_string());
        assert_eq!(result2, "281");
    }
}

