use std::collections::HashMap;
use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<_>>>();
    let width: isize = grid[0].len() as isize;
    let height: isize = grid.len() as isize;

    let node_positions: HashMap<(isize, isize), char> = grid.iter()
        .enumerate()
        .flat_map(|(y, &ref row)| row.iter()
            .enumerate()
            .filter(|(_, &c)| c != '.')
            .map(move |(x, &char)| ((x as isize, y as isize), char)))
        .collect();

    let nodes: HashMap<char, Vec<(isize, isize)>> = node_positions.values().map(|&v| {
        let vec: Vec<(isize, isize)> = node_positions.iter().filter(|&(_, &v1)| v1 == v ).map(|(&pos, _)| pos).collect();
        (v, vec)
    }).collect();

    let mut anti_nodes: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for (frequency, vec) in nodes {
        vec.iter()
            .tuple_combinations()
            .for_each(|(&a, &b)| {
                let shadow1 = (a.0 - (b.0 - a.0), a.1 - (b.1 - a.1));
                let shadow2 = (b.0 + (b.0 - a.0), b.1 + (b.1 - a.1));
                let shadow_list = anti_nodes.entry(frequency).or_insert_with(Vec::new);
                shadow_list.push(shadow1);
                shadow_list.push(shadow2);
            });
    }

    let anti_nodes: HashMap<char, Vec<(isize, isize)>> = anti_nodes.into_iter()
        .map(|(c, vec)| {
            (c,
             vec.into_iter()
                 .unique()
                 .filter(|&pos| pos.0 >= 0 && pos.1 >= 0 && pos.0 < width && pos.1 < height)
                 .collect::<Vec<_>>()
            )
        }).collect();

    let x = anti_nodes.values().flatten().unique().count();
    Some(x)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<_>>>();
    let width: isize = grid[0].len() as isize;
    let height: isize = grid.len() as isize;

    let node_positions: HashMap<(isize, isize), char> = grid.iter()
        .enumerate()
        .flat_map(|(y, &ref row)| row.iter()
            .enumerate()
            .filter(|(_, &c)| c != '.')
            .map(move |(x, &char)| ((x as isize, y as isize), char)))
        .collect();

    let nodes: HashMap<char, Vec<(isize, isize)>> = node_positions.values().map(|&v| {
        let vec: Vec<(isize, isize)> = node_positions.iter().filter(|&(_, &v1)| v1 == v ).map(|(&pos, _)| pos).collect();
        (v, vec)
    }).collect();

    let mut anti_nodes: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for (frequency, vec) in nodes {
        vec.iter()
            .tuple_combinations()
            .for_each(|(&a, &b)| {
                let shadow_list = anti_nodes.entry(frequency).or_insert_with(Vec::new);

                let offset = (b.0 - a.0, b.1 - a.1);
                let mut q = 0;
                while q < width {
                    shadow_list.push((a.0 - offset.0 * q, a.1 - offset.1 * q));
                    shadow_list.push((b.0 + offset.0 * q, b.1 + offset.1 * q));

                    q += 1;
                }
            });
    }

    let anti_nodes: HashMap<char, Vec<(isize, isize)>> = anti_nodes.into_iter()
        .map(|(c, vec)| {
            (c,
             vec.into_iter()
                 .unique()
                 .filter(|&pos| pos.0 >= 0 && pos.1 >= 0 && pos.0 < width && pos.1 < height)
                 .collect::<Vec<_>>()
            )
        }).collect();

    let x = anti_nodes.values().flatten().unique().count();
    Some(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
