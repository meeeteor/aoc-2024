use dashmap::DashMap;
use std::collections::HashMap;
use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let mut state: HashMap<usize, usize> = input.split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .into_group_map_by(|&v| v)
        .into_iter()
        .map(|(k, v)| (k, v.len()))
        .collect::<HashMap<_, _>>();

    for _ in 0..25 {
        state = next(state);
    }

    Some(state.into_iter().map(|(_, v)| v).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut state: HashMap<usize, usize> = input.split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .into_group_map_by(|&v| v)
        .into_iter()
        .map(|(k, v)| (k, v.len()))
        .collect::<HashMap<_, _>>();

    for _ in 0..75 {
        state = next(state);
    }

    Some(state.into_iter().map(|(_, v)| v).sum())
}

fn next(state: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let new_state: DashMap<usize, usize> = DashMap::new();
    state.into_par_iter().for_each(|(value, count)| {
        let num_digits = (value as f64).log10().floor() as u32 + 1;

        if value == 0 { new_state.entry(1).and_modify(|c| *c += count).or_insert(count); }
        else if num_digits % 2 == 0 {
            let (l, r) = split(value, num_digits);
            new_state.entry(l).and_modify(|c| *c += count).or_insert(count);
            new_state.entry(r).and_modify(|c| *c += count).or_insert(count);
        }
        else {
            new_state.entry(value*2024).and_modify(|c| *c += count).or_insert(count);
        }
    });

    new_state.into_iter().collect()
}

fn split(number: usize, num_digits: u32) -> (usize, usize) {
    let left = number / 10usize.pow(num_digits / 2);
    let right = number % 10usize.pow(num_digits / 2);
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
