extern crate num;

use num::integer::lcm;

type Vector = [i32; 3];

fn add(a: Vector, b: Vector) -> Vector {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn abs(v: Vector) -> i32 {
    v[0].abs() + v[1].abs() + v[2].abs()
}

#[derive(Debug)]
struct Moon {
    position: Vector,
    velocity: Vector,
}

fn compare(a: i32, b: i32) -> i32 {
    if a > b {
        -1
    } else if a < b {
        1
    } else {
        0
    }
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            position: [x, y, z],
            velocity: [0, 0, 0],
        }
    }

    fn apply_velocity(&mut self) {
        self.position = add(self.position, self.velocity);
    }

    fn apply_gravity(&mut self, other: Vector) {
        self.velocity[0] += compare(self.position[0], other[0]);
        self.velocity[1] += compare(self.position[1], other[1]);
        self.velocity[2] += compare(self.position[2], other[2]);
    }

    fn energy(&self) -> i32 {
        abs(self.position) * abs(self.velocity)
    }
}

fn main() {
    let mut moons = vec![
        Moon::new(-8, -18, 6),
        Moon::new(-11, -14, 4),
        Moon::new(8, -3, -10),
        Moon::new(-2, -16, 1),
        // Moon::new(-1, 0, 2),
        // Moon::new(2, -10, -7),
        // Moon::new(4, -8, 8),
        // Moon::new(3, 5, -1),
    ];

    // part 1

    simulate(&mut moons, 1000);

    // for i in 0..4 {
    //     println!("{:?}: {:?}", moons[i].position, moons[i].velocity);
    // }

    let mut sum = 0;
    for m in &moons {
        sum += m.energy();
    }

    println!("{}", sum);

    // part 2
    // credit to https://github.com/frerich/aoc2019/blob/master/rust/day12/src/main.rs

    simulate_until_repeat(&mut moons);
}

fn simulate(moons: &mut Vec<Moon>, num_steps: i32) {
    for _ in 0..num_steps {
        for i in 0..moons.len() {
            for j in 0..moons.len() {
                if i != j {
                    let other = moons[j].position;
                    moons[i].apply_gravity(other);
                }
            }
        }
        for i in 0..moons.len() {
            moons[i].apply_velocity();
        }
    }
}

fn dimension_state(moons: &[Moon], dim: usize) -> [i32; 8] {
    [
        moons[0].position[dim], moons[0].velocity[dim],
        moons[1].position[dim], moons[1].velocity[dim],
        moons[2].position[dim], moons[2].velocity[dim],
        moons[3].position[dim], moons[3].velocity[dim]
    ]
}

fn simulate_until_repeat(moons: &mut Vec<Moon>) {
    let initial_state = [
        dimension_state(&moons, 0),
        dimension_state(&moons, 1),
        dimension_state(&moons, 2)
    ];

    let mut cycle_len = [0, 0, 0];

    let mut counter: usize = 1;
    while cycle_len[0] == 0 || cycle_len[1] == 0 || cycle_len[2] == 0 {
        for i in 0..moons.len() {
            for j in 0..moons.len() {
                if i != j {
                    let other = moons[j].position;
                    moons[i].apply_gravity(other);
                }
            }
        }
        for i in 0..moons.len() {
            moons[i].apply_velocity();
        }
        for i in 0..3 {
            if dimension_state(&moons, i) == initial_state[i] {
                cycle_len[i] = counter;
            }
        }
        counter += 1;
    }

    println!("{:?}", lcm(cycle_len[0], lcm(cycle_len[1], cycle_len[2])));
}
