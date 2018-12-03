fn main() -> std::io::Result<()> {
    let input: &str = include_str!("../../tests/day02/input");

    let box_ids: Vec<&str> = input.split_whitespace().collect();

    println!("Part I: {}", part_one(box_ids));

    println!("Part II: {}", "");
    Ok(())
}

fn part_one(box_ids: Vec<&str>) -> i32 {
    let (mut two_count, mut three_count) = (0, 0);

    for id in &box_ids {
        let mut counts = [0u8; 26];

        // Turn the id into chars, from the chars turn them into an unsigned int between 1-26
        for letter_position in id.chars().map(|letter| letter as usize - 'a' as usize) {
            counts[letter_position] += 1;
        }

        // Count the occurrences and update the tuple.

        if counts.iter().any(|&i| i == 2) {
            two_count += 1;
        }

        if counts.iter().any(|&i| i == 3) {
            three_count += 1;
        }
    }

    return two_count * three_count;
}
