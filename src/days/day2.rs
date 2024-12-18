use itertools::Itertools;

fn parse_report(report: &str) -> Vec<i64> {
    report
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn is_report_safe(report: &[i64]) -> bool {
    let (d, lt, gt) = report
        .into_iter()
        .tuple_windows()
        .map(|(l, r)| ((l - r).abs(), l < r, l > r))
        .fold((true, true, true), |(rd, rlt, rgt), (d, lt, gt)| {
            (rd && d >= 1 && d <= 3, rlt && lt, rgt && gt)
        });

    d && (lt ^ gt)
}

#[aoc_runner_derive::aoc(day2, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_report)
        .filter(|report| is_report_safe(report))
        .count()
}

fn is_report_safe_with_tolerance(report: &[i64]) -> bool {
    if is_report_safe(report) {
        return true;
    }

    (0..report.len())
        .into_iter()
        .map(|ti| {
            report
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != ti)
                .map(|(_, &el)| el)
                .collect::<Vec<_>>()
        })
        .map(|report| is_report_safe(&report))
        .any(|is_safe| is_safe)
}

#[aoc_runner_derive::aoc(day2, part2)]
pub fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_report)
        .filter(|report| is_report_safe_with_tolerance(report))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1_sample() {
        assert_eq!(
            solve_part1("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9"),
            2
        );
    }

    #[test]
    fn test_solve_part2_sample() {
        assert_eq!(
            solve_part2("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9"),
            4
        );
    }
}
