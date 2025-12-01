use std::io::Result;

fn part2(x: u64) -> i128 {
    x.pow(2) as i128 - (x*51699878) as i128 + 377117112241505 as i128
}

pub fn run() -> Result<String> {
    let mut counter: u32 = 0;
    for input in 0..51699878 {
        if part2(input) < 0 {
            counter += 1;
        }
    }

    Ok(counter.to_string())
}