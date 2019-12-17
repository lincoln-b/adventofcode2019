use std::f64;
use std::collections::HashMap;
use std::cmp::{Eq, PartialOrd, Ord, Ordering};
use std::hash::{Hash, Hasher};

const MAP: &str = "\
.#.####..#.#...#...##..#.#.##.
..#####.##..#..##....#..#...#.
......#.......##.##.#....##..#
..#..##..#.###.....#.#..###.#.
..#..#..##..#.#.##..###.......
...##....#.##.#.#..##.##.#...#
.##...#.#.##..#.#........#.#..
.##...##.##..#.#.##.#.#.#.##.#
#..##....#...###.#..##.#...##.
.###.###..##......#..#...###.#
.#..#.####.#..#....#.##..#.#.#
..#...#..#.#######....###.....
####..#.#.#...##...##....#..##
##..#.##.#.#..##.###.#.##.##..
..#.........#.#.#.#.......#..#
...##.#.....#.#.##........#..#
##..###.....#.............#.##
.#...#....#..####.#.#......##.
..#..##..###...#.....#...##..#
...####..#.#.##..#....#.#.....
####.#####.#.#....#.#....##.#.
#.#..#......#.........##..#.#.
#....##.....#........#..##.##.
.###.##...##..#.##.#.#...#.#.#
##.###....##....#.#.....#.###.
..#...#......#........####..#.
#....#.###.##.#...#.#.#.#.....
.........##....#...#.....#..##
###....#.........#..#..#.#.#..
##...#...###.#..#.###....#.##.
";

#[derive(Debug)]
struct Loc {
    can_see: usize,
    x: i32,
    y: i32,
}

impl Loc {
    fn distance(&self, other: &Loc) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}

impl PartialEq for Loc {
    fn eq(&self, other: &Loc) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Loc {}

struct Scout {
    asteroids: Vec<Loc>,
}

#[derive(Debug, Copy, Clone)]
struct Float(f64);

impl Float {
    fn canonicalize(&self) -> i64 {
        (self.0 * 1024.0 * 1024.0).round() as i64
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Float) -> bool {
        self.canonicalize() == other.canonicalize()
    }
}

impl Ord for Float {
    fn cmp(&self, other: &Float) -> Ordering {
        if self.0 < other.0 {
            Ordering::Less
        } else if self.0 > other.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Float) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Float {}

impl Hash for Float {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.canonicalize().hash(state);
    }
}

fn main() {
    // part 1
    let scout = Scout::new(&MAP);
    let station = scout.find_station();
    println!("{:?}", station);

    // part 2
    let vaporize = scout.vaporize(&station);
    println!("{:?}", vaporize[199]);
}

impl Scout {
    fn new(input: &str) -> Scout {
        let mut asteroids = Vec::new();
        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    asteroids.push(Loc { can_see: 0, x: j as i32, y: i as i32 });
                }
            }
        }
        Scout { asteroids }
    }

    fn vaporize(&self, start: &Loc) -> Vec<&Loc> {
        let mut results = Vec::new();
        let angles = self.count_unique_angles(start);
        let mut collected: Vec<&Float> = angles.keys().collect();
        collected.sort();
        collected.reverse();
        let mut change = true;
        while change {
            change = false;
            for angle in &collected {
                if let Some(locs) = angles.get(angle) {
                    for loc in locs {
                        if !results.contains(loc) {
                            change = true;
                            results.push(loc);
                            break;
                        }
                    }
                }
            }
        }
        results
    }

    fn find_station(&self) -> Loc {
        let mut best = Loc {
            can_see: 0,
            x: 0,
            y: 0,
        };
        for asteroid in &self.asteroids {
            let can_see = self.count_unique_angles(&asteroid).keys().count();
            // println!("> asteroid {:?} can see {}", asteroid, can_see);
            if can_see > best.can_see {
                best.can_see = can_see;
                best.x = asteroid.x;
                best.y = asteroid.y;
            }
        }
        best
    }

    fn count_unique_angles(&self, asteroid: &Loc) -> HashMap<Float, Vec<&Loc>> {
        let mut uniq = HashMap::new();
        for other in &self.asteroids {
            if asteroid.x != other.x || asteroid.y != other.y {
                let x = (asteroid.x - other.x) as f64;
                let y = (asteroid.y - other.y) as f64;
                let mut key = Float(x.atan2(y));
                if key.0 > 0.0 {
                    key.0 -= 4.0 * f64::consts::FRAC_PI_2;
                }
                let e = uniq.entry(key).or_insert(vec![]);
                // if asteroid.x == 2 && asteroid.y == 2 {
                //     println!("{:?} ({}, {}): {:?}", other, x, y, key);
                // }
                e.push(other);
                e.sort_by(|a, b| a.distance(asteroid).cmp(&b.distance(asteroid)));
            }
        }
        // if asteroid.x == 2 && asteroid.y == 2 {
        //     let mut keys: Vec<&Float> = uniq.keys().collect();
        //     keys.sort();
        //     keys.reverse();
        //     println!("{:?}", keys);
        // }
        uniq
    }
}
