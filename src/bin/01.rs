use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = interpret_input(input);

    left.sort();
    right.sort();

    let x = left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    Some(x)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right): (Vec<u32>, Vec<u32>) = interpret_input(input);

    let counts = right.iter().counts();

    let x = left.iter()
        .map(|v| *v * (*counts.get(v).unwrap_or(&0)) as u32)
        .sum();

    Some(x)
}

fn interpret_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input.lines().map(|row| {
        let mut parts = row.split_whitespace();

        (parts.next().unwrap().parse::<u32>().unwrap(), parts.next().unwrap().parse::<u32>().unwrap())
    }).unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
