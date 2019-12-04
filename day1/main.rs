use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn lines_from_file(filename: String) -> Result<Vec<i32>, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?.parse::<i32>().unwrap());
    }
    Ok(lines)
}

fn part1() {
    let nums = lines_from_file("input.txt".to_string()).unwrap();
    let mut sum = 0;
    for num in nums {
        sum += num / 3 - 2;
    }

    println!("{}", sum);
}

fn calculate_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn part2() {
    let nums = lines_from_file("input.txt".to_string()).unwrap();
    let mut sum = 0;
    for num in nums {
        let mut fuelsum = 0;
        let mut lastpart = num;
        let mut fuelpart = calculate_fuel(lastpart);
        while fuelpart > 0 {
            fuelsum += fuelpart;
            lastpart = fuelpart;
            fuelpart = calculate_fuel(lastpart);
        }
        sum += fuelsum;
    }

    println!("{}", sum);
}

fn main() {
    // part1();
    part2();
}
