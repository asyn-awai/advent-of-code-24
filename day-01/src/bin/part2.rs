fn main() {
    let input = include_str!("./input.txt");

    let data: Vec<(u32, u32)> = input
        .lines()
        .filter_map(|pair| {
            let mut pair = pair.splitn(2, "   ");

            Some((pair.next()?.parse().ok()?, pair.next()?.parse().ok()?))
        })
        .collect();

    let (left, right): (Vec<_>, Vec<_>) = data.iter().cloned().unzip();

    let similarity_score: u32 = left
        .iter()
        .map(|l| l * right.iter().filter(|&r| r == l).count() as u32)
        .sum();

    println!("{}", similarity_score);
}
