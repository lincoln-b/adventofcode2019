pub struct IntCode {
    memory: Vec<i32>,
    counter: usize,
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

impl IntCode {

    pub fn new(mem: &[i32]) -> IntCode {
        IntCode {
            memory: mem.to_vec(),
            counter: 0,
        }
    }

    fn get(&self, mode: Mode, index: usize) -> i32 {
        match mode {
            Mode::Position => self.memory[self.memory[index] as usize],
            Mode::Immediate => self.memory[index],
        }
    }

    pub fn run<'a, I: Iterator<Item=&'a i32>>(&mut self, mut input: I) -> Option<i32> {
        while self.counter < self.memory.len() {
            match Instruction::parse(self.memory[self.counter]) {
                Instruction::Add(a, b) => {
                    let dest = self.memory[self.counter + 3] as usize;
                    self.memory[dest] = self.get(a, self.counter + 1) + self.get(b, self.counter + 2);
                    self.counter += 4;
                }
                Instruction::Multiply(a, b) => {
                    let dest = self.memory[self.counter + 3] as usize;
                    self.memory[dest] = self.get(a, self.counter + 1) * self.get(b, self.counter + 2);
                    self.counter += 4;
                }
                Instruction::Input => {
                    let i1 = self.memory[self.counter + 1] as usize;
                    self.memory[i1] = *input.next().unwrap();
                    self.counter += 2
                }
                Instruction::Output(a) => {
                    let val = Some(self.get(a, self.counter + 1));
                    self.counter += 2;
                    return val;
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
                Instruction::LessThan(a, b) => {
                    let i3 = self.memory[self.counter + 3] as usize;
                    if self.get(a, self.counter + 1) < self.get(b, self.counter + 2) {
                        self.memory[i3] = 1;
                    } else {
                        self.memory[i3] = 0;
                    }
                    self.counter += 4;
                }
                Instruction::Equals(a, b) => {
                    let i3 = self.memory[self.counter + 3] as usize;
                    if self.get(a, self.counter + 1) == self.get(b, self.counter + 2) {
                        self.memory[i3] = 1;
                    } else {
                        self.memory[i3] = 0;
                    }
                    self.counter += 4;
                }
                Instruction::Break => {
                    break;
                }
            };
        }
        None
    }
}

