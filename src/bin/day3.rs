use regex::Regex;
use std::fs;

fn match_and_calc_all(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
        })
        .sum()
}

fn match_and_calc_enabled(input: &str) -> i32 {
    let re = Regex::new(r"(do\(\))|(mul\(\d+,\d+\))|(don't\(\))").unwrap();
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;
    let mut is_enabled = true;
    for (_, [s]) in re.captures_iter(input).map(|c| c.extract()) {
        if s == "do()" {
            is_enabled = true;
            continue;
        } else if s == "don't()" {
            is_enabled = false;
            continue;
        } else {
            if is_enabled {
                let (_, [a, b]) = re_mul.captures(s).unwrap().extract();
                result += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {
        let example = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(161, match_and_calc_all(example));
        assert_eq!(48, match_and_calc_enabled(example));
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day3.txt").unwrap();

    let r1 = match_and_calc_all(&input);
    println!("[Part 1] Result = {r1}");

    let r2 = match_and_calc_enabled(&input);
    println!("[Part 2] Result = {r2}");
}
