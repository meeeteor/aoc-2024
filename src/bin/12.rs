use std::collections::HashSet;
use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(12);

struct Grid {
    fields: Vec<Vec<char>>,
    groups: Vec<Vec<(usize, usize)>>,
    width: usize,
    height: usize,

}
impl Grid {
    fn from_string(string: &str) -> Self {
        let mut grid = Grid{fields: Vec::new(), groups: Vec::new(), width: 0, height: 0};

        grid.fields = string.lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        grid.height = grid.fields.len();
        grid.width = grid.fields[0].len();

        grid
    }

    fn find_groups(&mut self) {
        let mut visited = vec![vec![false; self.width]; self.height];

        self.groups = (0..self.height).cartesian_product(0..self.width)
            .filter_map(|(i, j)| {
                if visited[i][j] { None }
                else {
                    let group = self.dfs(&mut visited, i, j, self.fields[i][j]);
                    Some(group)
                }
            })
            .collect();
    }

    fn dfs(&self, visited: &mut Vec<Vec<bool>>, i: usize, j: usize, ch: char) -> Vec<(usize, usize)> {
        if i >= self.height || j >= self.width || visited[i][j] || self.fields[i][j] != ch {
            return vec![];
        }

        visited[i][j] = true;
        let mut group = vec![(i,j)];

        for (di, dj) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_i = i as isize + di;
            let new_j = j as isize + dj;
            if new_i >= 0 && new_j >= 0 {
                group.extend(self.dfs(visited, new_i as usize, new_j as usize, ch));
            }
        }

        group
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = Grid::from_string(input);
    grid.find_groups();

    let x = grid.groups.par_iter()
        .map(|group| {
            let mut perimeter = 0;

            let directions = [(0, 1), (1, 0), (-1, 0), (0, -1)];
            for &(i, j) in group {
                for &(di, dj) in directions.iter() {
                    let new_i = i as isize + di;
                    let new_j = j as isize + dj;

                    if new_j < 0 || new_j >= grid.width as isize ||
                        new_i < 0 || new_j >= grid.height as isize ||
                        !group.contains(&(new_i as usize, new_j as usize))
                    {
                        perimeter += 1;
                    }
                }
            }

            perimeter * group.len()
        })
        .sum();



    Some(x)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = Grid::from_string(input);
    grid.find_groups();
    println!("{:?}", grid.groups);
    let grid = Grid {fields: Vec::new(), groups: vec![vec![(0, 0), (0, 1), (0, 2)]], width: 0, height: 0};

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}


/*
let mut edges: HashSet<(isize, isize, isize, isize)> = HashSet::new();
            let directions = [(0, 1), (1, 0), (-1, 0), (0, -1)];
            for &(i, j) in group {
                for &(di, dj) in directions.iter() {
                    let new_i = i as isize + di;
                    let new_j = j as isize + dj;

                    if new_j < 0 || new_j >= grid.width as isize ||
                        new_i < 0 || new_j >= grid.height as isize ||
                        !group.contains(&(new_i as usize, new_j as usize))
                    {
                        edges.insert((i as isize, j as isize, new_i, new_j));
                    }
                }
            }

            for edge in edges.clone().into_iter() {
                if edge.0 == edge.2 {
                    // horizontal
                    iterate_h(&mut edges, edge);
                } else {
                    // vertical
                    iterate_v(&mut edges, edge);
                }
                edges.insert(edge);
            }
            println!("{:?}", edges.len());

            return edges.len() * group.len();

            fn iterate_h(edges: &mut HashSet<(isize, isize, isize, isize)>, edge: (isize, isize, isize, isize))
            {
                edges.remove(&edge);
                if edges.contains(&(edge.0, edge.1 - 1, edge.2, edge.3 - 1)) {
                    iterate_h(edges, (edge.0, edge.1 - 1, edge.2, edge.3 - 1))
                }
                if edges.contains(&(edge.0, edge.1 + 1, edge.2, edge.3 + 1)) {
                    iterate_h(edges, (edge.0, edge.1 + 1, edge.2, edge.3 + 1))
                }
                if edges.contains(&(edge.2, edge.3 + 1, edge.0, edge.1 + 1)) {
                    iterate_h(edges, (edge.2, edge.3 + 1, edge.0, edge.1 + 1))
                }
                if edges.contains(&(edge.2, edge.3 - 1, edge.0, edge.1 - 1)) {
                    iterate_h(edges, (edge.2, edge.3 - 1, edge.0, edge.1 - 1))
                }
            }

            fn iterate_v(edges: &mut HashSet<(isize, isize, isize, isize)>, edge: (isize, isize, isize, isize))
            {
                edges.remove(&edge);
                if edges.contains(&(edge.0 - 1, edge.1, edge.2 - 1, edge.3)) {
                    iterate_v(edges, (edge.0 - 1, edge.1, edge.2 - 1, edge.3))
                }
                if edges.contains(&(edge.0 + 1, edge.1, edge.2 + 1, edge.3)) {
                    iterate_v(edges, (edge.0 + 1, edge.1, edge.2 + 1, edge.3))
                }
                if edges.contains(&(edge.2+1, edge.3, edge.0 + 1, edge.1)) {
                    iterate_v(edges, (edge.2+1, edge.3, edge.0 + 1, edge.1))
                }
                if edges.contains(&(edge.2-1, edge.3, edge.0 - 1, edge.1)) {
                    iterate_v(edges, (edge.2-1, edge.3, edge.0 - 1, edge.1))
                }

            }
 */