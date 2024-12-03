use itertools::Itertools;
use regex::Regex;

#[aoc_runner_derive::aoc(day3, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    re.find_iter(&input.lines().join(""))
        .map(|m| m.as_str())
        .map(|m| m.split_once(',').unwrap())
        .map(|(l, r)| {
            (
                (&l[4..]).parse::<i32>().unwrap(),
                (&r[..r.len() - 1]).parse::<i32>().unwrap(),
            )
        })
        .map(|(l, r)| l * r)
        .sum()
}

#[aoc_runner_derive::aoc(day3, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let re = Regex::new(r"don't\(\).*?(do\(\)|$)").unwrap();

    let input = input.lines().join("");
    let input = re.replace_all(&input, "");

    solve_part1(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1_sample() {
        assert_eq!(
            solve_part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn test_solve_part2_sample() {
        assert_eq!(
            solve_part2(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ),
            48
        );
    }
}
