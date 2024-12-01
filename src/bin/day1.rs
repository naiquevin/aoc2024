use std::{collections::HashMap, fs};

fn part_1(left: &[i32], right: &[i32]) {
    let pairs = left.iter().zip(right.iter());
    let mut total_distance = 0;
    for (a, b) in pairs {
        total_distance += i32::abs(a - b)
    }
    println!("[Part 1] Total distance: {total_distance}");
}

fn freq_dist(xs: &[i32]) -> HashMap<i32, i32> {
    let mut result: HashMap<i32, i32> = HashMap::new();
    for x in xs {
        result.entry(*x).and_modify(|n| *n += 1).or_insert(1);
    }
    result
}

fn part_2(left: &[i32], right: &[i32]) {
    let right_freqs = freq_dist(right);
    let mut result = 0;
    for x in left {
        result += x * right_freqs.get(x).unwrap_or(&0);
    }
    println!("[Part 2] Result: {result}");
}

fn main() {
    let input = fs::read_to_string("inputs/day1.txt").unwrap();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let parts = line
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        left.push(parts[0]);
        right.push(parts[1]);
    }
    left.sort();
    right.sort();
    part_1(&left, &right);
    part_2(&left, &right);
}
