use std::collections::HashMap;
use itertools::Itertools;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<_>>>();
    let width: isize = grid[0].len() as isize;
    let height: isize = grid.len() as isize;

    let grid_positions: HashMap<(isize, isize), usize> = grid.iter()
        .enumerate()
        .flat_map(|(y, &ref row)| row.iter()
            .enumerate()
            .map(move |(x, &char)| ((x as isize, y as isize), char.to_digit(10).unwrap() as usize)))
        .collect();

    let mut heads: HashMap<(isize, isize), usize> = grid_positions.iter()
        .filter(|(_, &v)| v == 0 )
        .map(|(&(x, y), &v)| ((x, y), v))
        .collect();

    for (&head_position, _) in heads.clone().iter() {
        let ends = walk(head_position, &grid_positions, (width, height));
        let end_count = ends.iter().unique().count();

        heads.insert(head_position, end_count);
    }

    let x = heads.values().sum();

    Some(x)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<_>>>();
    let width: isize = grid[0].len() as isize;
    let height: isize = grid.len() as isize;

    let grid_positions: HashMap<(isize, isize), usize> = grid.iter()
        .enumerate()
        .flat_map(|(y, &ref row)| row.iter()
            .enumerate()
            .map(move |(x, &char)| ((x as isize, y as isize), char.to_digit(10).unwrap() as usize)))
        .collect();

    let mut heads: HashMap<(isize, isize), usize> = grid_positions.iter()
        .filter(|(_, &v)| v == 0 )
        .map(|(&(x, y), &v)| ((x, y), v))
        .collect();

    for (&head_position, _) in heads.clone().iter() {
        let ends = walk(head_position, &grid_positions, (width, height));
        let end_count = ends.iter().count();

        heads.insert(head_position, end_count);
    }

    let x = heads.values().sum();

    Some(x)
}

fn walk(position: (isize, isize), nodes: &HashMap<(isize, isize), usize>, bounds: (isize, isize))
    -> Vec<(isize, isize)>
{
    if nodes[&position] == 9 {return vec![position];}

    let mut vec: Vec<(isize, isize)> = Vec::new();
    for dir in vec![(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let new_pos = (position.0 + dir.0, position.1 + dir.1);
        if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= bounds.0 || new_pos.1 >= bounds.1 { continue; }
        if nodes[&new_pos] == nodes[&position] + 1 {
            vec.append(&mut walk(new_pos, &nodes, bounds));
        }
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
