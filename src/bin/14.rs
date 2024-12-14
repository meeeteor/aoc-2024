use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(14);

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

fn to_isize(str: &str) -> isize { str.parse::<isize>().unwrap() }

pub fn part_one(input: &str) -> Option<usize> {
    let x_center = (WIDTH - 1)/ 2; // 0 1 2 3 4 *5* 6 7 8 9 10
    let y_center = (HEIGHT - 1) / 2; // 0 1 2 *3* 4 5 6

    let pattern = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let robots = input.lines()
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            let x = to_isize(&captures[1]);
            let y = to_isize(&captures[2]);
            let dx = to_isize(&captures[3]);
            let dy = to_isize(&captures[4]);

            let mut nx = (x + 100*dx) % WIDTH;
            if nx < 0 { nx = nx + WIDTH }
            let mut ny = (y + 100*dy) % HEIGHT;
            if ny < 0 { ny = ny + HEIGHT }

            (nx, ny)
        }).collect::<Vec<_>>();

    let q1 = robots.iter()
        .filter(|&robot| robot.0 < x_center && robot.1 < y_center).count();
    let q2 = robots.iter()
        .filter(|&robot| robot.0 > x_center && robot.1 < y_center).count();
    let q3 = robots.iter()
        .filter(|&robot| robot.0 < x_center && robot.1 > y_center).count();
    let q4 = robots.iter()
        .filter(|&robot| robot.0 > x_center && robot.1 > y_center).count();

    Some(q1*q2*q3*q4)
}

#[derive(Debug)]
struct Robot {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

impl Robot {
    fn step(&mut self) {
        self.x = (self.x + self.dx) % WIDTH;
        if self.x < 0 { self.x = self.x + WIDTH }

        self.y = (self.y + self.dy) % HEIGHT;
        if self.y < 0 { self.y = self.y + HEIGHT }
    }
}

fn display(robots: Vec<Robot>) {
    let map: HashMap<(isize, isize), usize> = robots.iter().map(|robot| (robot.x, robot.y)).counts();

    println!("{}", map.values().sum::<usize>() == 500); // true!

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let c = map.get(&(x, y));
            if c.is_some() {
                print!("{}", c.unwrap());
            } else {
                print!(".");
            }
        }
        println!();
    }




}

pub fn part_two(input: &str) -> Option<usize> {
    let pattern = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = input.lines()
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            let x = to_isize(&captures[1]);
            let y = to_isize(&captures[2]);
            let dx = to_isize(&captures[3]);
            let dy = to_isize(&captures[4]);

            Robot { x, y, dx, dy }
        }).collect::<Vec<_>>();

    let mut steps = 0;
    loop {
        steps += 1;
        robots.iter_mut().for_each(|robot| robot.step());

        if !robots.iter().tuple_combinations().any(|(a, b)| a.x == b.x && a.y == b.y) {
            println!("{:?} {:?}", robots.len(), robots.iter().any(|r| (r.x < 0 || r.y < 0)));
            println!("{:?} {:?}", robots.len(), robots.iter().any(|r| (r.x >= WIDTH || r.y >= HEIGHT)));

            display(robots);
            break;
        }

    }

    Some(steps)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // REMEMBER TO SET WIDTH AND HEIGHT CORRECTLY!
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
