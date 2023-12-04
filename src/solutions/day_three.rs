use crate::utils;

#[derive(Debug, PartialEq)]
enum SchematicCoordinate {
    Number(u32),
    Empty,
    Symbol(char),
}

#[derive(Debug, PartialEq)]
struct Coord(i32, i32);
struct Schematic(Vec<Vec<SchematicCoordinate>>);
impl Schematic {
    fn parse_schematic(lines: &[String]) -> Schematic {
        let out = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => SchematicCoordinate::Empty,
                        '0'..='9' => SchematicCoordinate::Number(c.to_digit(10).unwrap()),
                        _ => SchematicCoordinate::Symbol(c),
                    })
                    .collect::<Vec<SchematicCoordinate>>()
            })
            .collect::<Vec<Vec<SchematicCoordinate>>>();
        Schematic(out)
    }

    fn adjacent_coords(&self, coord: &Coord) -> Vec<Coord> {
        let (row, col) = (coord.0, coord.1);
        let hops = Vec::from([
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]);
        let possibles = hops.iter().map(|(r, c)| (row + r, col + c));
        possibles
            .filter(|(r, c)| {
                !(*r < 0 || *c < 0 || *r >= self.0.len() as i32 || *c >= self.0[0].len() as i32)
            })
            .map(|(r, c)| Coord(r, c))
            .collect::<Vec<Coord>>()
    }

    fn get_from_coord(&self, coord: &Coord) -> Option<&SchematicCoordinate> {
        let (row, col) = (coord.0, coord.1);
        if row < 0 || col < 0 || row >= self.0.len() as i32 || col >= self.0[0].len() as i32 {
            None
        } else {
            Some(&self.0[row as usize][col as usize])
        }
    }

    fn has_symbol_around(&self, row: usize, col: usize) -> bool {
        let adjacents = self.adjacent_coords(&Coord(row as i32, col as i32));
        adjacents.iter().any(|coord| {
            matches!(
                self.get_from_coord(coord),
                Some(SchematicCoordinate::Symbol(_))
            )
        })
    }

    fn get_number(&self, row: usize, col: usize) -> Option<u32> {
        match self.0[row][col] {
            SchematicCoordinate::Number(n) => {
                // scan left and right, collecting all digits until we hit a
                // symbol or empty space. Then return as a number
                let mut digits = Vec::from([n]);
                let mut col_i = col as i32 - 1;
                while col_i >= 0 {
                    match self.0[row][col_i as usize] {
                        SchematicCoordinate::Number(n) => {
                            digits.insert(0, n);
                            col_i -= 1;
                        }
                        _ => break,
                    }
                }
                let mut col_i = col + 1;
                while col_i < self.0[row].len() {
                    match self.0[row][col_i] {
                        SchematicCoordinate::Number(n) => {
                            digits.push(n);
                            col_i += 1;
                        }
                        _ => break,
                    }
                }
                Some(digits.iter().fold(0, |acc, n| acc * 10 + n))
            }
            _ => None,
        }
    }
}

pub fn run(test_mode: bool) {
    let lines = utils::read_day_as_lines(3, test_mode);
    let schematic = Schematic::parse_schematic(&lines);
    println!("Part one: {}", part_one(&schematic));
    println!("Part two: {}", part_two(&schematic));
}
fn part_one(schematic: &Schematic) -> u32 {
    let mut row_idx = 0;
    let mut part_nums: Vec<u32> = Vec::new();
    while row_idx < schematic.0.len() {
        let mut col_idx = 0;
        while col_idx < schematic.0[row_idx].len() {
            match (
                schematic.get_number(row_idx, col_idx),
                schematic.has_symbol_around(row_idx, col_idx),
            ) {
                (Some(n), true) => {
                    // println!("Found number {} which has symbol around", n);
                    part_nums.push(n);
                    while col_idx < schematic.0[row_idx].len()
                        && schematic.get_number(row_idx, col_idx).is_some()
                    {
                        col_idx += 1;
                    }
                }
                _ => col_idx += 1,
            }
        }
        row_idx += 1;
    }
    part_nums.iter().sum()
}

fn part_two(schematic: &Schematic) -> u32 {
    let mut sum = 0;
    schematic.0.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter().enumerate().for_each(|(col_idx, col)| {
            if let SchematicCoordinate::Symbol('*') = col {
                let coord = Coord(row_idx as i32, col_idx as i32);
                let adjacent_coords = schematic.adjacent_coords(&coord);
                let mut nums = adjacent_coords
                    .iter()
                    .filter(|coord| {
                        matches!(
                            schematic.get_from_coord(coord),
                            Some(SchematicCoordinate::Number(_))
                        )
                    })
                    .map(|coord| {
                        schematic
                            .get_number(coord.0 as usize, coord.1 as usize)
                            .unwrap()
                    })
                    .collect::<Vec<u32>>();
                nums.sort();
                nums.dedup();
                if nums.len() == 2 {
                    sum += nums.iter().product::<u32>();
                }
            }
        })
    });
    sum
}
