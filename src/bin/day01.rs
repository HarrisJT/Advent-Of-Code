use std::collections::HashSet;

fn main() {
    let input: &str = include_str!("../../tests/day01/input");
    let frequency_changes: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    println!("Part I: {}", frequency_changes.iter().sum::<i32>());

    let mut seen_frequencies = HashSet::new();
    let mut frequency_sum = 0;

    // Find the first result that is not None and map it to final
    frequency_changes
        .iter()
        .cycle()
        .find_map(|i| {
            frequency_sum += i;
            seen_frequencies.replace(frequency_sum)
        }).expect("No repeated frequency sums.");

    println!("Part II: {}", frequency_sum);
}
