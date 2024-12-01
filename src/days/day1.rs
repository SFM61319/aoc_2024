#[aoc_runner_derive::aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let line_count = input.lines().count();

    let mut left_list = Vec::<i64>::with_capacity(line_count);
    let mut right_list = Vec::<i64>::with_capacity(line_count);

    input
        .lines()
        .map(str::split_whitespace)
        .map(|mut numbers| (numbers.next().unwrap(), numbers.next().unwrap()))
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .for_each(|(a, b)| {
            left_list.push(a);
            right_list.push(b);
        });

    left_list.sort();
    right_list.sort();

    left_list
        .into_iter()
        .zip(right_list.into_iter())
        .map(|(a, b)| a - b)
        .map(i64::abs)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1_sample() {
        assert_eq!(solve_part1("3   4\n4   3\n2   5\n1   3\n3   9\n3   3"), 11)
    }
}
