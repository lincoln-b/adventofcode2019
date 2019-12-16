mod input;
mod program;

use program::IntCode;

fn main() {

    // phase 1

    let permutations = calculate_permutations(vec![0, 1, 2, 3, 4]);
    let mut max = 0;
    for p in &permutations {
        let val = run_program_with_settings(p);
        if val > max {
            max = val;
        }
    }
    println!("{:?}", max);

    // phase 2

    let permutations = calculate_permutations(vec![5, 6, 7, 8, 9]);
    let mut max = 0;
    for p in permutations {
        let val = loop_program_with_settings(&p);
        if val > max {
            max = val;
        }
    }
    println!("{:?}", max);
}

fn loop_program_with_settings(settings: &Vec<i32>) -> i32 {
    let mut last_out = 0;
    let mut computers = vec![
        IntCode::new(&input::ORIGINAL),
        IntCode::new(&input::ORIGINAL),
        IntCode::new(&input::ORIGINAL),
        IntCode::new(&input::ORIGINAL),
        IntCode::new(&input::ORIGINAL),
    ];
    for (i, phase) in settings.iter().enumerate() {
        last_out = computers[i].run([*phase,last_out].iter()).unwrap();
    }
    let mut i = 0;
    loop {
        last_out = match computers[i].run([last_out].iter()) {
            Some(v) => v,
            None => {
                return last_out;
            }
        };
        i = if i == 4 { 0 } else { i + 1 };
    }
}

fn run_program_with_settings(settings: &Vec<i32>) -> i32 {
    let mut last_out = 0;
    for phase in settings {
        last_out = IntCode::new(&input::ORIGINAL).run([*phase,last_out].iter()).unwrap();
    }
    last_out
}

fn calculate_permutations(vec: Vec<i32>) -> Vec<Vec<i32>> {
    if vec.len() <= 1 {
        return vec![vec];
    }

    let mut out: Vec<Vec<i32>> = Vec::new();
    for (i, x) in vec.iter().enumerate() {
        let mut vec_cpy = vec.clone();
        vec_cpy.remove(i);
        for v in calculate_permutations(vec_cpy) {
            let mut vec_cpy = vec![*x];
            vec_cpy.extend(v);
            out.push(vec_cpy);
        }
    }
    out
}
