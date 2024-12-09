use std::iter::repeat;
use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let mut vec = parse_input(input);
    compact(&mut vec);
    let x = vec.iter()
        .filter(|v| v.is_some())
        .enumerate()
        .map(|(i,v)| i * v.unwrap())
        .sum();

    Some(x)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut vec = parse_input(input);
    compact_preserve_size(&mut vec);

    let x: usize = vec.iter()
        .enumerate()
        .filter_map(|(i,&v)| {
            if v.is_none() {return None}
            Some(i * v.unwrap())
        })
        .sum();

    Some(x)
}


fn parse_input(input: &str) -> Vec<Option<usize>> {
    let line = input.lines().next().unwrap_or("");
    let mut vec: Vec<Option<usize>> = Vec::new();
    let mut id = 0;
    for (idx, char) in line.chars().enumerate() {
        let digit = char.to_digit(10).unwrap() as usize;
        if idx%2==0 {
            vec.extend(repeat(Some(id)).take(digit));
            id += 1;
        } else {
            vec.extend(repeat(None).take(digit));
        }
    }

    vec
}

fn compact(vec: &mut Vec<Option<usize>>) {
    let mut left = 0;
    let mut right = vec.len()-1;

    while left < right {
        while left < vec.len() && vec[left].is_some() {
            left += 1;
        }
        while right > 0 && vec[right].is_none() {
            right -= 1;
        }

        if left < right {
            vec.swap(left, right);
        }
    }
}

fn compact_preserve_size(vec: &mut Vec<Option<usize>>) {
    let mut idx = vec.iter().rev().find(|p|p.is_some()).unwrap().unwrap();

    while idx > 0 {
        let right = vec.iter().positions(|&p| p == Some(idx)).collect::<Vec<_>>();

        let mut left = 0;
        while left < vec.len() && (vec[left].is_some() || get_size(&vec, left) < right.len()) {
            left += 1;
        }

        if left < right[0] && get_size(&vec, left) >= right.len() {
            for (i, &p) in right.iter().enumerate() {
                vec.swap(left+i, p);
            }
        }

        idx -= 1;
    }

    fn get_size(vec: &Vec<Option<usize>>, left: usize) -> usize {
        let mut size = 1;
        while (left + size) < vec.len() && vec[left + size] == vec[left] {
            size += 1;
        }
        size
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
