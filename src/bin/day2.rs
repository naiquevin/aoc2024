use std::fs;

fn parse_report(line: &str) -> Vec<i32> {
    line.split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn is_gradual_delta(a: i32, b: i32) -> bool {
    let delta = i32::abs(a - b);
    delta >= 1 && delta <= 3
}

fn is_in_order(is_incr: bool, curr: i32, prev: i32) -> bool {
    if is_incr {
        curr > prev
    } else {
        curr < prev
    }
}

fn check_report(report: &Vec<i32>) -> bool {
    let mut prev: Option<i32> = None;
    let mut is_incr: Option<bool> = None;
    for x in report {
        let curr = *x;
        match (prev, is_incr) {
            (None, None) => {
                prev = Some(*x);
                continue;
            }
            (None, Some(_)) => unreachable!(),
            (Some(p), None) => {
                if !is_gradual_delta(curr, p) {
                    return false;
                }
                is_incr = Some(curr > p);
                prev = Some(curr);
            }
            (Some(p), Some(incr)) => {
                if !is_in_order(incr, curr, p) || !is_gradual_delta(curr, p) {
                    return false;
                }
                prev = Some(curr);
            }
        }
    }
    true
}

fn check_report_with_dampener(report: &Vec<i32>) -> bool {
    let mut prev: Option<i32> = None;
    let mut is_incr: Option<bool> = None;
    let mut num_problems = 0;
    for (i, x) in report.iter().enumerate() {
        let curr = *x;
        // println!("prev={prev:?}; curr={curr}; is_incr={is_incr:?}; probs={num_problems}");
        match (prev, is_incr) {
            (None, None) => {
                prev = Some(*x);
                continue;
            }
            (None, Some(_)) => unreachable!(),
            (Some(p), None) => {
                if !is_gradual_delta(curr, p) {
                    num_problems += 1;
                    if num_problems > 1 {
                        return false;
                    }

                    // Check if we can skip the current item
                    if is_gradual_delta(report[i + 1], p) {
                        continue;
                    }
                }
                is_incr = Some(curr > p);
                prev = Some(curr);
            }
            (Some(p), Some(incr)) => {
                if !is_in_order(incr, curr, p) || !is_gradual_delta(curr, p) {
                    num_problems += 1;
                    if num_problems > 1 {
                        return false;
                    }

                    // Here we need to specially handle 2 cases when
                    // dealing with the 3rd item in the list where it
                    // doesn't conform to the order already
                    // established by the first two items
                    //
                    //   1) does removing the 1st item and recomputing
                    //      the order help?
                    //
                    //   2) does removing the 2nd item and recomputing
                    //      the order help?
                    //
                    if i == 2 {
                        if is_gradual_delta(curr, p) && is_in_order(curr > p, report[i + 1], curr) {
                            is_incr = Some(curr > p);
                            prev = Some(curr);
                            continue;
                        }

                        if is_gradual_delta(curr, report[i - 2])
                            && is_in_order(curr > report[i - 2], curr, report[i - 2])
                        {
                            is_incr = Some(curr > report[i - 2]);
                            prev = Some(curr);
                            continue;
                        }
                    }

                    // For any other case, we skip curr by not updating
                    // prev i.e. by doing nothing!
                } else {
                    prev = Some(curr);
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_report() {
        assert!(check_report(&vec![7, 6, 4, 2, 1]));
        assert!(!check_report(&vec![1, 2, 7, 8, 9]));
        assert!(!check_report(&vec![9, 7, 6, 2, 1]));
        assert!(!check_report(&vec![1, 3, 2, 4, 5]));
        assert!(!check_report(&vec![8, 6, 4, 4, 1]));
        assert!(check_report(&vec![1, 3, 6, 7, 9]));
    }

    #[test]
    fn test_check_report_with_dampener() {
        assert!(check_report_with_dampener(&vec![7, 6, 4, 2, 1]));
        assert!(!check_report_with_dampener(&vec![1, 2, 7, 8, 9]));
        assert!(!check_report_with_dampener(&vec![9, 7, 6, 2, 1]));
        assert!(check_report_with_dampener(&vec![1, 3, 2, 4, 5]));
        assert!(check_report_with_dampener(&vec![8, 6, 4, 4, 1]));
        assert!(check_report_with_dampener(&vec![1, 3, 6, 7, 9]));

        assert!(!check_report_with_dampener(&vec![84, 84, 88, 89, 89]));

        // When removing the 2nd item may work but order changes
        assert!(check_report_with_dampener(&vec![4, 2, 3, 4, 5]));

        // When only the last item results in a violation
        assert!(check_report_with_dampener(&vec![1, 2, 3, 4, 2]));

        assert!(check_report_with_dampener(&vec![70, 68, 72, 74, 75, 78]));

        assert!(!check_report_with_dampener(&vec![54, 53, 58, 60, 62]));
    }
}

fn main() {
    let num_safe = fs::read_to_string("inputs/day2.txt")
        .unwrap()
        .lines()
        .map(parse_report)
        .filter(check_report)
        .count();
    println!("Num safe = {num_safe}");

    let num_safe_with_dampener = fs::read_to_string("inputs/day2.txt")
        .unwrap()
        .lines()
        .map(parse_report)
        .filter(check_report_with_dampener)
        .count();
    println!("Num safe with dampener = {num_safe_with_dampener}");
}
