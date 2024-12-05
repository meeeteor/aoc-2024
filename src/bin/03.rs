use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re1 = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap(); // find valid instructions
    let re2 = Regex::new(r"\d+").unwrap(); //find digits in string

    let result = re1.find_iter(input)
        .map(|m| {
            let numbers: Vec<u32> = re2.find_iter(m.as_str())
                .take(2)
                .filter_map(|m2| m2.as_str().parse().ok())
                .collect();

            numbers[0] * numbers[1]
        }).sum::<u32>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re1 = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let re2 = Regex::new(r"\d+").unwrap();
    let dont_idxs = input.match_indices("don't()").map(|(i, _)| i).collect::<Vec<_>>();
    let do_idxs = input.match_indices("do()").map(|(i, _)| i).collect::<Vec<_>>();

    let result = re1.find_iter(input)
        .filter(|m| {
            let position = m.start();
            dont_idxs.iter().rev()
                .find(|&&idx| idx < position)           // find all donts before the current match
                .map_or(true, |&dont_position| {            // no don't before: keep; if found:
                    do_idxs.iter().rev()
                        .find(|&&idx| idx > dont_position && idx < position) // find if there is a do between the dont and the current match
                        .is_some()      // if there is a do between the dont and the match, keep.
                })
        })
        .map(|m| {
            let numbers: Vec<u32> = re2.find_iter(m.as_str())
                .take(2)
                .filter_map(|m2| m2.as_str().parse().ok())
                .collect();

            numbers[0] * numbers[1]
        }).sum::<u32>();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY,2));
        assert_eq!(result, Some(48));
    }
}
