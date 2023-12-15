use std::{fs::File, io::{BufReader, BufRead, Result}};

struct RangeMap {
    source_start: i64,
    destination_start: i64,
    width: i64,
}

impl RangeMap {
    fn translate(&self, input: i64) -> i64 {
        if input < self.source_start || input >= (self.source_start + self.width) {
            return input;
        }

        return input + (self.destination_start - self.source_start);
    }
}

fn process_step(seeds: &mut Vec<i64>, ranges: &mut Vec<RangeMap>) -> () {
    for seed in seeds {
        let seed_value = *seed;

        for range in ranges.iter() {
            if seed_value != range.translate(seed_value) {
                *seed = range.translate(seed_value);
                break;
            }
            *seed = range.translate(seed_value);
        }
    }
}

pub fn run() -> Result<String> {
    let filepath = "./src/day5/input.txt";
    let file = File::open(filepath);
    let reader = BufReader::new(file.unwrap());

    let mut seeds: Vec<i64> = Vec::new();
    let mut ranges: Vec<RangeMap> = Vec::new();

    for line in reader.lines() {
        let line_string = line.unwrap();

        if line_string.contains("seeds:") {
            let seed_ranges: Vec<i64> = line_string.split_whitespace().skip(1)
                .map(|seed| seed.parse::<i64>().unwrap())
                .collect();
            for i in 0..seed_ranges.len() {
                if i % 2 == 0 {
                    match seed_ranges.get(i + 1) {
                        Some(range_width) => {
                            seeds.extend(seed_ranges[i]..(*range_width+seed_ranges[i]));
                        },
                        None => continue,
                    }
                } else {
                    continue;
                }
            }

        } else if 2 == line_string.split_whitespace().count() {
            println!("Now starting: {}", line_string);

        } else if 3 == line_string.split_whitespace().count() {
            let range_input: Vec<i64> = line_string.split_whitespace()
                .map(|seed| seed.parse::<i64>().unwrap())
                .collect();
            ranges.push(RangeMap { 
                source_start: *range_input.get(1).unwrap(), 
                destination_start: *range_input.get(0).unwrap(),  
                width: *range_input.get(2).unwrap() 
            });

        } else if 0 == line_string.split_whitespace().count() {
            process_step(&mut seeds, &mut ranges);
            ranges = Vec::new();
        }
    }

    process_step(&mut seeds, &mut ranges);

    let answer = seeds.iter().min().unwrap();
    println!("Minimal location is: {}", answer);
    Ok(answer.to_string())
}
