use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let res = re
        .captures_iter(input)
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
        .sum::<u32>();

    println!("{}", res);
}
