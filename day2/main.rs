fn main() {
    let original = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 2, 9, 19, 23, 2, 13, 23, 27,
        1, 6, 27, 31, 2, 6, 31, 35, 2, 13, 35, 39, 1, 39, 10, 43, 2, 43, 13, 47, 1, 9, 47, 51, 1,
        51, 13, 55, 1, 55, 13, 59, 2, 59, 13, 63, 1, 63, 6, 67, 2, 6, 67, 71, 1, 5, 71, 75, 2, 6,
        75, 79, 1, 5, 79, 83, 2, 83, 6, 87, 1, 5, 87, 91, 1, 6, 91, 95, 2, 95, 6, 99, 1, 5, 99,
        103, 1, 6, 103, 107, 1, 107, 2, 111, 1, 111, 5, 0, 99, 2, 14, 0, 0,
    ];

    // part 1
    let mut part1_memory = original.clone();
    run_program(&mut part1_memory);
    println!("{}", part1_memory[0]);

    // part 2
    let needle = 19690720;
    for x in 0..99 {
        for y in 0..99 {
            let mut memory = original.clone();
            memory[1] = x;
            memory[2] = y;
            run_program(&mut memory);
            if memory[0] == needle {
                println!("{}, {}", x, y);
                break;
            }
        }
    }
    
}

fn run_program(memory: &mut Vec<i32>) {
    let mut i = 0;
    while i < memory.len() {
        match memory[i] {
            1 | 2 => {
                let opcode = memory[i];
                let s1 = memory[i + 1] as usize;
                let s2 = memory[i + 2] as usize;
                let dest = memory[i + 3] as usize;
                memory[dest] = if opcode == 1 { memory[s1] + memory[s2] } else { memory[s1] * memory[s2] };
                i += 4;
            },
            99 => break,
            _ => i += 1,
        }
    }
}
