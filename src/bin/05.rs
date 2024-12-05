use std::collections::HashMap;
use itertools::Itertools;

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
    let mut rule_map = HashMap::new();
    rules.iter().for_each(|(a,b)| rule_map.entry(a).or_insert_with(Vec::new).push(b));

    let invalid_updates = updates.iter()
        .filter(|update| update.iter()
            .tuple_combinations()
            .any(|(a, b)| {
                match rule_map.get(b) {
                    Some(pages) => pages.contains(  &a),
                    None => false
                }
            }))
        .collect::<Vec<_>>();

    let sum: i32 = invalid_updates.iter()
        .map(|&update| {
            let mut update = update.clone();
            let mut swapped = true;
            while swapped {
                swapped = false;
                let mut new_update = update.clone();
                update.iter().enumerate().tuple_combinations().for_each(|((a_idx, a_val), (b_idx, b_val))| {
                    match rule_map.get(b_val) {
                        Some(rule) => {
                            if rule.contains(&a_val) {
                                new_update.swap(a_idx, b_idx);
                                swapped=true;
                            }
                        }
                        None => {}
                    }
                    // if let mut rule = rule_map.get(b_val) {
                    //     if rule.iter().contains(a_val) {
                    //         new_update.swap(*a_idx, b_idx);
                    //         swapped=true;
                    //     }
                    // }
                });
                update = new_update;
            }
            update[update.len() / 2] as i32
        }).sum();

    Some(sum as u32)
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
