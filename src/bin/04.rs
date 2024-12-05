advent_of_code::solution!(4);

#[derive(Debug)]
struct Field{
    letter: char,
}
impl Field{
    fn new(letter: char) -> Self {
        Field{letter}
    }
}

struct Grid{
    fields: Vec<Vec<Field>>,
    rows: usize,
    cols: usize,
}

impl Grid{
    fn from_string(string: &str) -> Self {
        let rows: Vec<&str> = string.lines().collect();
        let fields: Vec<Vec<Field>> = rows.iter()
            .map(|row| row.chars().map(Field::new).collect())
            .collect();

        Grid{ fields, rows: rows.len(), cols: rows[0].len() }
    }

    fn check_direction(&self, row: usize, col: usize, dir: (i32, i32), pattern: &str) -> bool {
        let mut row = row as i32;
        let mut col = col as i32;

        for char in pattern.chars() {
            //check out of bounds:
            if row < 0 || row >= self.rows as i32 || col < 0 || col >= self.cols as i32 { return false; }

            //check character
            if self.fields[row as usize][col as usize].letter != char { return false; }

            //go to next position
            row += dir.0;
            col += dir.1;
        }
        true
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_string(input);
    let pattern = "XMAS";

    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1),];

    let mut count = 0;
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            count += directions.iter()
                .filter(|dir| grid.check_direction(row, col, **dir, pattern))
                .count() as u32;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_string(input);

    let mut count = 0;
    for row in 1..grid.rows-1 {
        for col in 1..grid.cols-1 {
            if grid.fields[row][col].letter != 'A' { continue };

            if
                (
                    grid.check_direction(row-1, col-1, (1, 1), "MAS") ||
                    grid.check_direction(row+1, col+1, (-1, -1), "MAS")
                )
                &&
                (
                    grid.check_direction(row-1, col+1, (1, -1), "MAS") ||
                    grid.check_direction(row+1, col-1, (-1, 1), "MAS")
                ) {
                count += 1;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
