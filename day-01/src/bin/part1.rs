fn main() {
    let input = include_str!("./input.txt");

    let data: Vec<(i32, i32)> = input
        .lines()
        .filter_map(|pair| {
            let mut pair = pair.splitn(2, "   ");

            Some((pair.next()?.parse().ok()?, pair.next()?.parse().ok()?))
        })
        .collect();

    let (mut left, mut right): (Vec<_>, Vec<_>) = data.iter().cloned().unzip();
    left.sort();
    right.sort();

    let sum_of_distances: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).unsigned_abs())
        .sum();

    println!("{}", sum_of_distances);
}
