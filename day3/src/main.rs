use std::cmp;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Other,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i32,
}

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn lines_from_file(filename: String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

fn instructions_from_line(line: &str) -> Vec<Instruction> {
    let mut vec = Vec::new();
    for item in line.split(',') {
        let d = match item.as_bytes()[0] as char {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => Direction::Other,
        };
        let x = item[1..].parse::<i32>().unwrap();
        vec.push(Instruction {
            direction: d,
            distance: x,
        });
    }
    vec
}

fn coordinates_from_instructions(instructions: &[Instruction]) -> Vec<Coordinate> {
    let mut vec = Vec::new();
    let mut x = 0;
    let mut y = 0;
    vec.push(Coordinate { x, y });
    for instruction in instructions {
        x += match instruction.direction {
            Direction::Left => -instruction.distance,
            Direction::Right => instruction.distance,
            _ => 0,
        };
        y += match instruction.direction {
            Direction::Down => -instruction.distance,
            Direction::Up => instruction.distance,
            _ => 0,
        };
        vec.push(Coordinate { x, y });
    }
    vec
}

fn find_intersection(
    line1_start: &Coordinate,
    line1_end: &Coordinate,
    line2_start: &Coordinate,
    line2_end: &Coordinate,
) -> Result<Coordinate, ()> {
    let x1_between = line1_start.x >= cmp::min(line2_start.x, line2_end.x)
        && line1_start.x <= cmp::max(line2_start.x, line2_end.x);
    let x2_between = line2_start.x >= cmp::min(line1_start.x, line1_end.x)
        && line2_start.x <= cmp::max(line1_start.x, line1_end.x);
    let y1_between = line1_start.y >= cmp::min(line2_start.y, line2_end.y)
        && line1_start.y <= cmp::max(line2_start.y, line2_end.y);
    let y2_between = line2_start.y >= cmp::min(line1_start.y, line1_end.y)
        && line2_start.y <= cmp::max(line1_start.y, line1_end.y);
    if x1_between && y2_between {
        Ok(Coordinate {
            x: line1_start.x,
            y: line2_start.y,
        })
    } else if x2_between && y1_between {
        Ok(Coordinate {
            x: line2_start.x,
            y: line1_start.y,
        })
    } else {
        Err(())
    }
}

fn find_collisions(
    coordinates1: &[Coordinate],
    coordinates2: &[Coordinate],
) -> Vec<Coordinate> {
    let mut vec = Vec::new();
    for c1 in 1..coordinates1.len() {
        for c2 in 1..coordinates2.len() {
            let intersection = find_intersection(
                &coordinates1[c1],
                &coordinates1[c1 - 1],
                &coordinates2[c2],
                &coordinates2[c2 - 1],
            );
            if let Ok(v) = intersection {
                vec.push(v);
            }
        }
    }
    vec
}

fn find_closest_collision(collisions: &[Coordinate]) -> i32 {
    let mut min: i32 = -1;
    for collision in collisions {
        let dist = collision.x.abs() + collision.y.abs();
        if dist != 0 && (min == -1 || dist < min) {
            min = dist;
        }
    }
    min
}

fn find_first_collision(coordinates1: &[Coordinate], coordinates2: &[Coordinate]) -> i32 {
    let mut steps_c1 = 0;
    let mut steps_c2;
    for c1 in 1..coordinates1.len() {
        steps_c1 += (coordinates1[c1].x - coordinates1[c1 - 1].x).abs()
            + (coordinates1[c1].y - coordinates1[c1 - 1].y).abs();
        steps_c2 = 0;
        for c2 in 1..coordinates2.len() {
            steps_c2 += (coordinates2[c2].x - coordinates2[c2 - 1].x).abs()
                + (coordinates2[c2].y - coordinates2[c2 - 1].y).abs();
            let intersection = find_intersection(
                &coordinates1[c1],
                &coordinates1[c1 - 1],
                &coordinates2[c2],
                &coordinates2[c2 - 1],
            );
            if let Ok(v) = intersection {
                if v.x != 0 || v.y != 0 {
                    steps_c1 -=
                        (v.x - coordinates1[c1].x).abs() + (v.y - coordinates1[c1].y).abs();
                    steps_c2 -=
                        (v.x - coordinates2[c2].x).abs() + (v.y - coordinates2[c2].y).abs();
                    return steps_c1 + steps_c2;
                }
            }
        }
    }
    0
}

fn main() {
    // part 1
    let lines = lines_from_file("data.txt".to_string()).unwrap();
    let wire1_instructions = instructions_from_line(&lines[0]);
    let wire2_instructions = instructions_from_line(&lines[1]);
    let coordinates1 = coordinates_from_instructions(&wire1_instructions);
    let coordinates2 = coordinates_from_instructions(&wire2_instructions);
    let collisions = find_collisions(&coordinates1, &coordinates2);
    let closest = find_closest_collision(&collisions);
    println!("{}", closest);

    // part 2
    let closest = find_first_collision(&coordinates1, &coordinates2);
    println!("{}", closest);
}
