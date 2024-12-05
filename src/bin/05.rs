use std::collections::HashMap;
use itertools::Itertools;

// use itertools::Itertools;
advent_of_code::solution!(5);



pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = interpret_input(input);
    let x:i32 = updates.iter().map(|update| {
        // map page to position in update pages
        let positions: HashMap<usize, usize> = update.iter()
            .enumerate()
            .map(|(i, &v)| (v, i)).collect();

        // check whether update is valid
        let valid_update = rules.iter()
            .all(|&(node_a, node_b)| {
                !positions.contains_key(&node_a) || !positions.contains_key(&node_b) || positions[&node_a] < positions[&node_b]
            });
        if !valid_update {return 0 }
        update[update.len() / 2] as i32
    }).sum();

    Some(x as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = interpret_input(input);
    let rulemap: HashMap<usize, usize> = rules.iter().collect();

    let x:i32 = updates.iter().map(|update| {
        // map page to position in update pages
        let positions: HashMap<usize, usize> = update.iter()
            .enumerate()
            .map(|(i, &v)| (v, i)).collect();

        // check whether update is valid
        let valid_update = rules.iter()
            .all(|&(node_a, node_b)| {
                !positions.contains_key(&node_a) || !positions.contains_key(&node_b) || positions[&node_a] < positions[&node_b]
            });

        if !valid_update {
            println!("{:?}", update);

            let mut new_update = update.clone();
            new_update.iter().for_each(|page| {

            });

            println!("{:?}", update);
        }

        // update[update.len() / 2] as i32
        0
    }).sum();

    Some(x as u32)
}

fn interpret_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let rules = parts[0].lines()
        .filter_map(|line| {
            let values = line.split("|").collect::<Vec<&str>>();
            if values.len() != 2 {return None};

            Some((
                values[0].trim().parse::<usize>().ok()?,
                values[1].trim().parse::<usize>().ok()?,
                ))
        })
        .collect::<Vec<_>>();

    let updates = parts[1].lines()
        .map(|line| line.split(",")
            .filter_map(|v| v.parse::<usize>().ok())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    (rules, updates)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
