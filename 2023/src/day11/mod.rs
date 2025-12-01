use std::{fs::File, io::{BufReader, BufRead, Result}};

struct Galaxy {
    x: u64,
    y: u64,
}

impl Galaxy {
    fn calculate_distance(&self, target_galaxy: &Galaxy, empty_columns: &Vec<u64>, empty_rows: &Vec<u64>) -> u64 {
        let empty_columns_between = match self.x < target_galaxy.x {
            true => empty_columns.iter()
                .filter(|column| (self.x..target_galaxy.x).contains(*column))
                .count() as u64,
            false => empty_columns.iter()
                .filter(|column| (target_galaxy.x..self.x).contains(*column))
                .count() as u64,
        };
        let empty_rows_between = match self.y < target_galaxy.y {
            true => empty_rows.iter()
                .filter(|row| (self.y..target_galaxy.y).contains(*row))
                .count() as u64,
            false => empty_columns.iter()
                .filter(|row| (target_galaxy.y..self.y).contains(*row))
                .count() as u64,
        };

        let extra_columns: u64 = (1000000 - 1) * empty_columns_between;
        let extra_rows: u64 = (1000000 - 1) * empty_rows_between;
        
        self.x.abs_diff(target_galaxy.x) + self.y.abs_diff(target_galaxy.y) + extra_columns + extra_rows
    }
}

pub fn run() -> Result<String> {
    let filepath = "./src/day11/input.txt";
    let file = File::open(filepath);
    let reader = BufReader::new(file.unwrap());

    let mut total_distances: u64 = 0; 

    let mut galaxies = Vec::<Galaxy>::new();
    let mut empty_columns = Vec::<u64>::new();
    let mut empty_rows = Vec::<u64>::new();


    for (y, line) in reader.lines().enumerate() {
        let y = y as u64;
        let mut is_row_empty = true;

        for (x, point) in line?.chars().enumerate() {
            let x = x as u64;
            match point {
                '#' => {
                    if empty_columns.contains(&x) {
                        empty_columns.retain(|column| *column != x);
                    }

                    galaxies.push(Galaxy {x: x, y: y});
                    is_row_empty = false;
                },
                '.' => {
                    if 0 == y {
                        empty_columns.push(x);
                    }
                    continue;
                },
                _ => println!("UFO detected: {}", point),
            }
        }

        if is_row_empty {
            empty_rows.push(y)
        }
    }

    let galaxy_count = galaxies.len();
    for (i, galaxy) in galaxies.iter().enumerate() {
        for next_galaxy_count in (i+1)..galaxy_count {
            match galaxies.get(next_galaxy_count) {
                Some(other_galaxy) => total_distances += galaxy.calculate_distance(other_galaxy, &empty_columns, &empty_rows) as u64,
                None => println!("Uh oh"),
            }
        }
    }

    Ok(total_distances.to_string())
}
