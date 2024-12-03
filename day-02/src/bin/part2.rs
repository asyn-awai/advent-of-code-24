const PROBLEM_THRESHOLD: u32 = 1;

fn main() {
    let input = include_str!("input.txt");

    let reports = input
        .lines()
        .filter_map(|report| {
            let levels = report
                .split_whitespace()
                .map(|level| level.parse().ok())
                .collect::<Option<Vec<i32>>>()?;

            Some(levels)
        })
        .collect::<Vec<_>>();

    let num_safe_reports = reports
        .iter()
        .filter(|report| {
            let is_increasing = report[0] < report[1];

            let problems = report
                .windows(2)
                .filter(|window| {
                    let diff = window[1] - window[0];
                    !(1..=3).contains(&diff.abs()) || is_increasing != (diff > 0)
                })
                .count();

            problems as u32 <= PROBLEM_THRESHOLD
        })
        .count();

    println!("{}", num_safe_reports);
}
