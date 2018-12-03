use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let input: &str = include_str!("../../tests/day01/input");
    // Turn input into vector of integers
    let frequency_changes: Vec<i32> = input
        .split_whitespace()
        // Parse the string into i32 integer using turbofish because parse is too general, needs
        // to know what type to parse into. Unwrap returns value inside Result of parse.
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    println!("Part I: {}", frequency_changes.iter().sum::<i32>());

    let mut seen_frequencies = HashSet::new();
    let mut frequency_sum = 0;

    // Find the first result that is not None and continue when .replace returns Some
    frequency_changes
        .iter()
        .cycle()
        .find_map(|i| {
            frequency_sum += i;
            seen_frequencies.replace(frequency_sum)
        }).expect("No repeated frequency sums.");

    println!("Part II: {}", frequency_sum);
    Ok(())
}
