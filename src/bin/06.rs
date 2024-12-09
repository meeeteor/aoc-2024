use rayon::prelude::*;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let directions: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut direction = 0;

    let mut lines: Vec<String> = input.lines().map(|line|line.to_string()).collect::<Vec<_>>();

    let n_lines = lines.len();
    let n_cols = lines[0].len();

    let mut guard: (i32, i32) = lines.iter().enumerate()
        .find_map(|(row, line)| {
            line.chars()
                .position(|ch| ch == '^')
                .map(|col| (row as i32, col as i32))
        }).unwrap();
    lines[guard.0 as usize].replace_range((guard.1 as usize)..((guard.1 + 1) as usize), "X"); // Mark current position as visited

    loop {
        // check out of bounds
        if guard.0 + directions[direction].0 < 0 ||
            guard.0 + directions[direction].0 >= n_lines as i32 ||
            guard.1 + directions[direction].1 < 0 ||
            guard.1 + directions[direction].1 >= n_cols as i32 {

            let x = lines.iter()
                .map(|line| line.chars().filter(|&ch| ch == 'X').count() as u32).sum();
            return Some(x);
        }

        if lines[(guard.0 + directions[direction].0) as usize].chars().nth((guard.1 + directions[direction].1) as usize).unwrap() == '#' {
            // obstacle, so turn
            direction += 1;
            direction %= 4;
        } else {
            // no obstacle, so walk
            guard = (guard.0 + directions[direction].0, guard.1 + directions[direction].1);
            lines[guard.0 as usize].replace_range((guard.1 as usize)..((guard.1 + 1) as usize), "X");
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<String> = input.lines().map(|line|line.to_string()).collect::<Vec<_>>();

    let count: i32 = (0..lines.len())
        .into_par_iter()
        .map(|i|
            (0..lines[i].len())
                .filter(|&j| {
                    let char = lines[i].chars().nth(j).unwrap();
                    char != '^' && char != '#'
                })
                .map(|j| {
                    let mut clone = lines.clone();
                    clone[i].replace_range(j..(j+1), "#");
                    simulate(clone) as i32
                })
                .sum::<i32>())
        .sum();

    return Some(count as u32);


    // Returns whether the guard finds a loop
    fn simulate(mut lines: Vec<String>) -> bool {
        let mut history: Vec<((i32, i32), usize)> = Vec::new();

        let n_lines = lines.len();
        let n_cols = lines[0].len();

        let directions: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut direction = 0;

        let mut guard: (i32, i32) = lines.iter().enumerate()
            .find_map(|(row, line)| {
                line.chars()
                    .position(|ch| ch == '^')
                    .map(|col| (row as i32, col as i32))
            }).unwrap();
        lines[guard.0 as usize].replace_range((guard.1 as usize)..((guard.1 + 1) as usize), "X"); // Mark current position as visited

        loop {
            // check out of bounds
            if guard.0 + directions[direction].0 < 0 ||
                guard.0 + directions[direction].0 >= n_lines as i32 ||
                guard.1 + directions[direction].1 < 0 ||
                guard.1 + directions[direction].1 >= n_cols as i32 {

                return false;
            }

            if lines[(guard.0 + directions[direction].0) as usize].chars().nth((guard.1 + directions[direction].1) as usize).unwrap() == '#' {
                // obstacle, so turn
                direction += 1;
                direction %= 4;

                if history.iter().any(|&((row, col), dir)| {
                    row == guard.0 &&
                    col == guard.1 &&
                        dir == direction
                }) {
                    return true;
                } else {
                    history.push((guard, direction))
                }


            } else {
                // no obstacle, so walk
                guard = (guard.0 + directions[direction].0, guard.1 + directions[direction].1);
                lines[guard.0 as usize].replace_range((guard.1 as usize)..((guard.1 + 1) as usize), "X");
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
