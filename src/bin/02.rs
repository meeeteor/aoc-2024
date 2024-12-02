use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let values = interpret_input(input);

    let x = values.iter()
        .filter(|row| is_valid_row(row)).count();

    Some(x as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let values = interpret_input(input);

    let x = values.iter()
        .filter(|row| {
            if is_valid_row(row) {return true;}

            //any valid row when removing one?
            (0..row.len()).any(|i| {
                let mut modified_row = row.to_vec();
                modified_row.remove(i);

                is_valid_row(&&modified_row)
            })
        }).count();

    Some(x as u32)
}

fn is_valid_row(row: &&Vec<u32>) -> bool {
    let increasing = row.iter()
        .tuple_windows()
        .all(|(a, b)| a < b);
    let decreasing = row.iter()
        .tuple_windows()
        .all(|(a, b)| a > b);

    let valid_difference = row.iter()
        .tuple_windows()
        .all(|(a, b)| {
            let dif = a.abs_diff(*b);
            dif >= 1 && dif <= 3
        });

    (increasing || decreasing) && valid_difference
}

fn interpret_input(input: &str) -> Vec<Vec<u32>> {
    let rows = input.lines().collect::<Vec<&str>>();
    let values = rows.iter()
        .map(|row| row.split_whitespace()
            .filter_map(|num| num.parse::<u32>().ok())
            .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
