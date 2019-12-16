pub struct IntCode {
    memory: Vec<i64>,
    counter: usize,
    relative_base: i64,
}

#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug)]
enum Instruction {
    Add(Mode, Mode, Mode),
    Multiply(Mode, Mode, Mode),
    Input(Mode),
    Output(Mode),
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode, Mode),
    Equals(Mode, Mode, Mode),
    AdjustRelative(Mode),
    Break,
}

impl Instruction {
    fn parse(opcode: i64) -> Instruction {
        let mut opcode = opcode;
        let mut first_mode = Mode::Position;
        let mut second_mode = Mode::Position;
        let mut third_mode = Mode::Position;
        if opcode >= 10_000 {
            third_mode = if opcode / 10_000 == 1 {
                opcode -= 10_000;
                Mode::Immediate
            } else {
                opcode -= 20_000;
                Mode::Relative
            }
        }
        if opcode >= 1_000 {
            second_mode = if opcode / 1_000 == 1 {
                opcode -= 1_000;
                Mode::Immediate
            } else {
                opcode -= 2_000;
                Mode::Relative
            }
        }
        if opcode >= 100 {
            first_mode = if opcode / 100 == 1 {
                opcode -= 100;
                Mode::Immediate
            } else {
                opcode -= 200;
                Mode::Relative
            }
        }
        match opcode {
            1 => Instruction::Add(first_mode, second_mode, third_mode),
            2 => Instruction::Multiply(first_mode, second_mode, third_mode),
            3 => Instruction::Input(first_mode),
            4 => Instruction::Output(first_mode),
            5 => Instruction::JumpIfTrue(first_mode, second_mode),
            6 => Instruction::JumpIfFalse(first_mode, second_mode),
            7 => Instruction::LessThan(first_mode, second_mode, third_mode),
            8 => Instruction::Equals(first_mode, second_mode, third_mode),
            9 => Instruction::AdjustRelative(first_mode),
            99 => Instruction::Break,
            _ => panic!("opcode {} not recognized", opcode),
        }
    }
}

impl IntCode {
    pub fn new(mem: &[i64]) -> IntCode {
        IntCode {
            memory: mem.to_vec(),
            counter: 0,
            relative_base: 0,
        }
    }

    pub fn biggen(&mut self, size: usize) {
        self.memory.resize(size, 0);
    }

    fn get_index(&self, mode: Mode, index: usize) -> usize {
        match mode {
            Mode::Position => self.memory[index] as usize,
            Mode::Relative => (self.relative_base + self.memory[index]) as usize,
            _ => 0,
        }
    }

    fn get(&self, mode: Mode, index: usize) -> i64 {
        match mode {
            Mode::Position => self.memory[self.memory[index] as usize],
            Mode::Immediate => self.memory[index],
            Mode::Relative => self.memory[(self.relative_base + self.memory[index]) as usize],
        }
    }

    pub fn run<'a, I: Iterator<Item = &'a i64>>(&mut self, mut input: I) {
        while self.counter < self.memory.len() {
            match Instruction::parse(self.memory[self.counter]) {
                Instruction::Add(a, b, c) => {
                    let dest = self.get_index(c, self.counter + 3);
                    self.memory[dest] =
                        self.get(a, self.counter + 1) + self.get(b, self.counter + 2);
                    self.counter += 4;
                }
                Instruction::Multiply(a, b, c) => {
                    let dest = self.get_index(c, self.counter + 3);
                    self.memory[dest] =
                        self.get(a, self.counter + 1) * self.get(b, self.counter + 2);
                    self.counter += 4;
                }
                Instruction::Input(a) => {
                    let i1 = self.get_index(a, self.counter + 1);
                    self.memory[i1] = *input.next().unwrap();
                    self.counter += 2;
                }
                Instruction::Output(a) => {
                    let val = self.get(a, self.counter + 1);
                    self.counter += 2;
                    // return val;
                    println!("{}", val);
                }
                Instruction::JumpIfTrue(a, b) => {
                    if self.get(a, self.counter + 1) != 0 {
                        self.counter = self.get(b, self.counter + 2) as usize;
                    } else {
                        self.counter += 3;
                    }
                }
                Instruction::JumpIfFalse(a, b) => {
                    if self.get(a, self.counter + 1) == 0 {
                        self.counter = self.get(b, self.counter + 2) as usize;
                    } else {
                        self.counter += 3;
                    }
                }
                Instruction::LessThan(a, b, c) => {
                    let i3 = self.get_index(c, self.counter + 3);
                    if self.get(a, self.counter + 1) < self.get(b, self.counter + 2) {
                        self.memory[i3] = 1;
                    } else {
                        self.memory[i3] = 0;
                    }
                    self.counter += 4;
                }
                Instruction::Equals(a, b, c) => {
                    let i3 = self.get_index(c, self.counter + 3);
                    if self.get(a, self.counter + 1) == self.get(b, self.counter + 2) {
                        self.memory[i3] = 1;
                    } else {
                        self.memory[i3] = 0;
                    }
                    self.counter += 4;
                }
                Instruction::AdjustRelative(a) => {
                    self.relative_base += self.get(a, self.counter + 1);
                    self.counter += 2;
                }
                Instruction::Break => {
                    break;
                }
            };
        }
    }
}
