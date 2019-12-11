mod input;

struct Program {
    memory: [i32; 678],
}

fn main() {
    // phase 1
    let mut memory = Program{ memory: input::ORIGINAL };
    run_program(&mut memory, 1);

    // phase 2
    memory = Program{ memory: input::ORIGINAL };
    run_program(&mut memory, 5);
}

#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
}

#[derive(Debug)]
enum Instruction {
    Add(Mode, Mode),
    Multiply(Mode, Mode),
    Input,
    Output(Mode),
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode),
    Equals(Mode, Mode),
    Break,
}

impl Instruction {
    fn parse(opcode: i32) -> Instruction {
        let mut opcode = opcode;
        let mut first_mode = Mode::Position;
        let mut second_mode = Mode::Position;
        if opcode >= 1_000 {
            second_mode = Mode::Immediate;
            opcode -= 1_000;
        }
        if opcode >= 100 {
            first_mode = Mode::Immediate;
            opcode -= 100;
        }
        match opcode {
            1 => Instruction::Add(first_mode, second_mode),
            2 => Instruction::Multiply(first_mode, second_mode),
            3 => Instruction::Input,
            4 => Instruction::Output(first_mode),
            5 => Instruction::JumpIfTrue(first_mode, second_mode),
            6 => Instruction::JumpIfFalse(first_mode, second_mode),
            7 => Instruction::LessThan(first_mode, second_mode),
            8 => Instruction::Equals(first_mode, second_mode),
            99 => Instruction::Break,
            _ => panic!("opcode {} not recognized", opcode),
        }
    }
}

impl Program {
    fn get(&self, mode: Mode, index: usize) -> i32 {
        match mode {
            Mode::Position => self.memory[self.memory[index] as usize],
            Mode::Immediate => self.memory[index],
        }
    }
}

fn run_program(program: &mut Program, input: i32) {
    let mut i = 0;
    while i < program.memory.len() {
        match Instruction::parse(program.memory[i]) {
            Instruction::Add(a, b) => {
                let dest = program.memory[i + 3] as usize;
                program.memory[dest] = program.get(a, i + 1) + program.get(b, i + 2);
                i += 4;
            }
            Instruction::Multiply(a, b) => {
                let dest = program.memory[i + 3] as usize;
                program.memory[dest] = program.get(a, i + 1) * program.get(b, i + 2);
                i += 4;
            }
            Instruction::Input => {
                program.memory[program.memory[i + 1] as usize] = input;
                i += 2
            }
            Instruction::Output(a) => {
                println!("{}", program.get(a, i + 1));
                i += 2;
            }
            Instruction::JumpIfTrue(a, b) => {
                if program.get(a, i + 1) != 0 {
                    i = program.get(b, i + 2) as usize;
                } else {
                    i += 3;
                }
            }
            Instruction::JumpIfFalse(a, b) => {
                if program.get(a, i + 1) == 0 {
                    i = program.get(b, i + 2) as usize;
                } else {
                    i += 3;
                }
            }
            Instruction::LessThan(a, b) => {
                if program.get(a, i + 1) < program.get(b, i + 2) {
                    program.memory[program.memory[i + 3] as usize] = 1;
                } else {
                    program.memory[program.memory[i + 3] as usize] = 0;
                }
                i += 4;
            }
            Instruction::Equals(a, b) => {
                if program.get(a, i + 1) == program.get(b, i + 2) {
                    program.memory[program.memory[i + 3] as usize] = 1;
                } else {
                    program.memory[program.memory[i + 3] as usize] = 0;
                }
                i += 4;
            }
            Instruction::Break => {
                break;
            }
        };
    }
}
