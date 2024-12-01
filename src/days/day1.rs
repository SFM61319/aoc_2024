use ahash::AHashMap;

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

#[aoc_runner_derive::aoc(day1, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let line_count = input.lines().count();

    let mut left_list = Vec::<i64>::with_capacity(line_count);
    let mut right_counts = AHashMap::<i64, i64>::with_capacity(line_count);

    input
        .lines()
        .map(str::split_whitespace)
        .map(|mut numbers| (numbers.next().unwrap(), numbers.next().unwrap()))
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .for_each(|(a, b)| {
            left_list.push(a);
            right_counts
                .entry(b)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

    left_list
        .into_iter()
        .map(|a| a * right_counts.get(&a).copied().unwrap_or_default())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1_sample() {
        assert_eq!(solve_part1("3   4\n4   3\n2   5\n1   3\n3   9\n3   3"), 11);
    }

    #[test]
    fn test_solve_part2_sample() {
        assert_eq!(solve_part2("3   4\n4   3\n2   5\n1   3\n3   9\n3   3"), 31);
    }
}
