use std::cmp;

use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    let first_conditional_idx = cmp::min(
        input.find("do()").unwrap_or(input.len()),
        input.find("don't()").unwrap_or(input.len()),
    );

    let mut res = sum_mul_statements(&input[..first_conditional_idx]);
    let mut start = first_conditional_idx;

    while let Some(idx) = &input[start..].find("do()") {
        let p1 = start + idx + "do()".len();
        let p2 = input[p1..]
            .find("don't()")
            .map(|idx| idx + p1)
            .unwrap_or(input.len());

        let sub = &input[p1..p2];
        res += sum_mul_statements(sub);

        start = p2 + "don't()".len();
        if start >= input.len() {
            break;
        }
    }

    println!("{}", res);
}

fn sum_mul_statements(s: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(s)
        .filter_map(|capture| {
            let args = (capture.get(1), capture.get(2));

            match args {
                (Some(a), Some(b)) => {
                    let res = a.as_str().parse::<i32>().ok()? * b.as_str().parse::<i32>().ok()?;
                    Some(res as u32)
                }
                _ => None,
            }
        })
        .sum::<u32>()
}
