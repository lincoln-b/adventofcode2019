use std::fs;
use std::collections::HashMap;

fn build_orbit_map(filename: &str) -> HashMap<String, String> {
    let data = fs::read_to_string(filename).unwrap();
    let orbits: Vec<_> = data.trim().split('\n').collect();
    let mut orbit_map = HashMap::new();
    for line in &orbits {
        let mut iter = line.split(')');
        let center = iter.next().unwrap();
        let orbiter = iter.next().unwrap();
        orbit_map.insert(orbiter.to_string(), center.to_string());
    }
    orbit_map
}

fn main() {
    let orbit_map = build_orbit_map("data.txt");

    // part 1

    let mut num_orbits = 0;
    for v in orbit_map.values() {
        let mut needle = v;
        num_orbits += 1;
        while needle != "COM" {
            num_orbits += 1;
            needle = &orbit_map.get(needle).unwrap();
        }
    }
    println!("{}", num_orbits);

    // part 2
    
    let mut me_path = Vec::new();
    let mut needle: &str = &orbit_map.get("YOU").unwrap();
    while needle != "COM" {
        me_path.push(needle);
        needle = &orbit_map.get(needle).unwrap();
    }
    let mut santa_path = Vec::new();
    let mut needle: &str = &orbit_map.get("SAN").unwrap();
    while needle != "COM" {
        santa_path.push(needle);
        needle = &orbit_map.get(needle).unwrap();
    }
    let mut distance = 0;
    'outer: for (p1, star1) in me_path.iter().enumerate() {
        for (p2, star2) in santa_path.iter().enumerate() {
            if star1 == star2 {
                distance = p1 + p2;
                break 'outer;
            }
        }
    }
    println!("{}", distance);
}
