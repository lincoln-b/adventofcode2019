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

pub enum IoOperation {
    Read,
    Write(i64),
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
    fn parse_opcode(opcode: i64, base: i64) -> (i64, Mode) {
        let mut opcode = opcode;
        let mut mode = Mode::Position;
        if opcode >= base {
            if opcode / base == 1 {
                opcode -= base;
                mode = Mode::Immediate;
            } else {
                opcode -= 2 * base;
                mode = Mode::Relative;
            }
        }
        (opcode, mode)
    }
    
    fn parse(opcode: i64) -> Instruction {
        let (opcode, third_mode) = Instruction::parse_opcode(opcode, 10_000);
        let (opcode, second_mode) = Instruction::parse_opcode(opcode, 1_000);
        let (opcode, first_mode) = Instruction::parse_opcode(opcode, 100);

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
            Mode::Position => self.memory[self.counter + index] as usize,
            Mode::Relative => (self.relative_base + self.memory[self.counter + index]) as usize,
            _ => 0,
        }
    }

    fn get(&self, mode: Mode, index: usize) -> i64 {
        match mode {
            Mode::Position => self.memory[self.memory[self.counter + index] as usize],
            Mode::Immediate => self.memory[self.counter + index],
            Mode::Relative => self.memory[(self.relative_base + self.memory[self.counter + index]) as usize],
        }
    }

    pub fn run<'a, I: Iterator<Item = &'a i64>>(&mut self, mut input: I) -> Vec<i64> {
        let mut output = Vec::new();
        self.run_fn(|op| {
            match op {
                IoOperation::Read => *input.next().unwrap(),
                IoOperation::Write(val) => {
                    output.push(val);
                    0
                }
            }
        });
        output
    }

    pub fn run_fn<F>(&mut self, mut io: F)
    where
        F: FnMut(IoOperation) -> i64,
    {
        while self.counter < self.memory.len() {
            match Instruction::parse(self.memory[self.counter]) {
                Instruction::Add(a, b, c) => {
                    let dest = self.get_index(c, 3);
                    self.memory[dest] = self.get(a, 1) + self.get(b, 2);
                    self.counter += 4;
                }
                Instruction::Multiply(a, b, c) => {
                    let dest = self.get_index(c, 3);
                    self.memory[dest] = self.get(a, 1) * self.get(b, 2);
                    self.counter += 4;
                }
                Instruction::Input(a) => {
                    let i1 = self.get_index(a, 1);
                    self.memory[i1] = io(IoOperation::Read);
                    self.counter += 2;
                }
                Instruction::Output(a) => {
                    io(IoOperation::Write(self.get(a, 1)));
                    self.counter += 2;
                }
                Instruction::JumpIfTrue(a, b) => {
                    if self.get(a, 1) != 0 {
                        self.counter = self.get(b, 2) as usize;
                    } else {
                        self.counter += 3;
                    }
                }
                Instruction::JumpIfFalse(a, b) => {
                    if self.get(a, 1) == 0 {
                        self.counter = self.get(b, 2) as usize;
                    } else {
                        self.counter += 3;
                    }
                }
                Instruction::LessThan(a, b, c) => {
                    let i3 = self.get_index(c, 3);
                    if self.get(a, 1) < self.get(b, 2) {
                        self.memory[i3] = 1;
                    } else {
                        self.memory[i3] = 0;
                    }
                    self.counter += 4;
                }
                Instruction::Equals(a, b, c) => {
                    let i3 = self.get_index(c, 3);
                    if self.get(a, 1) == self.get(b, 2) {
                        self.memory[i3] = 1;
                    } else {
                        self.memory[i3] = 0;
                    }
                    self.counter += 4;
                }
                Instruction::AdjustRelative(a) => {
                    self.relative_base += self.get(a, 1);
                    self.counter += 2;
                }
                Instruction::Break => {
                    break;
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_quine() {
        let input = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut program = IntCode::new(&input);
        program.biggen(1000);
        let output = program.run([].iter());
        assert_eq!(input, output);
    }

    #[test]
    fn test_relative_1() {
        let input = vec![1102,34915192,34915192,7,4,7,99,0];
        let mut program = IntCode::new(&input);
        program.biggen(1000);
        let output = program.run([].iter());
        assert_eq!(output, vec![1219070632396864]);
    }

    #[test]
    fn test_relative_2() {
        let input = vec![104,1125899906842624,99];
        let mut program = IntCode::new(&input);
        program.biggen(1000);
        let output = program.run([].iter());
        assert_eq!(output, vec![1125899906842624]);
    }

    #[test]
    fn test_equals_eight() {
        let input = vec![3,9,8,9,10,9,4,9,99,-1,8];
        let output = IntCode::new(&input).run([7].iter());
        assert_eq!(output, vec![0]);

        let output = IntCode::new(&input).run([8].iter());
        assert_eq!(output, vec![1]);

        let output = IntCode::new(&input).run([9].iter());
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn test_less_than_eight() {
        let input = vec![3,9,7,9,10,9,4,9,99,-1,8];
        let output = IntCode::new(&input).run([7].iter());
        assert_eq!(output, vec![1]);

        let output = IntCode::new(&input).run([8].iter());
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn test_equals_eight_immediate() {
        let input = vec![3,3,1108,-1,8,3,4,3,99];
        let output = IntCode::new(&input).run([7].iter());
        assert_eq!(output, vec![0]);

        let output = IntCode::new(&input).run([8].iter());
        assert_eq!(output, vec![1]);

        let output = IntCode::new(&input).run([9].iter());
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn test_less_than_eight_immediate() {
        let input = vec![3,3,1107,-1,8,3,4,3,99];
        let output = IntCode::new(&input).run([7].iter());
        assert_eq!(output, vec![1]);

        let output = IntCode::new(&input).run([8].iter());
        assert_eq!(output, vec![0]);
    }
    
    #[test]
    fn test_jump_equals_zero() {
        let input = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let output = IntCode::new(&input).run([0].iter());
        assert_eq!(output, vec![0]);

        let output = IntCode::new(&input).run([1].iter());
        assert_eq!(output, vec![1]);
    }
    
    #[test]
    fn test_jump_equals_zero_immediate() {
        let input = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        let output = IntCode::new(&input).run([0].iter());
        assert_eq!(output, vec![0]);

        let output = IntCode::new(&input).run([1].iter());
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn test_equals_eight_long() {
        let input = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31, 1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104, 999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let output = IntCode::new(&input).run([3].iter());
        assert_eq!(output, vec![999]);

        let output = IntCode::new(&input).run([8].iter());
        assert_eq!(output, vec![1000]);

        let output = IntCode::new(&input).run([50].iter());
        assert_eq!(output, vec![1001]);
    }

    #[test]
    fn test_day5() {
        let input = vec![3,225,1,225,6,6,1100,1,238,225,104,0,1102,35,92,225,1101,25,55,225,1102,47,36,225,1102,17,35,225,1,165,18,224,1001,224,-106,224,4,224,102,8,223,223,1001,224,3,224,1,223,224,223,1101,68,23,224,101,-91,224,224,4,224,102,8,223,223,101,1,224,224,1,223,224,223,2,217,13,224,1001,224,-1890,224,4,224,102,8,223,223,1001,224,6,224,1,224,223,223,1102,69,77,224,1001,224,-5313,224,4,224,1002,223,8,223,101,2,224,224,1,224,223,223,102,50,22,224,101,-1800,224,224,4,224,1002,223,8,223,1001,224,5,224,1,224,223,223,1102,89,32,225,1001,26,60,224,1001,224,-95,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1102,51,79,225,1102,65,30,225,1002,170,86,224,101,-2580,224,224,4,224,102,8,223,223,1001,224,6,224,1,223,224,223,101,39,139,224,1001,224,-128,224,4,224,102,8,223,223,101,3,224,224,1,223,224,223,1102,54,93,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1008,677,677,224,1002,223,2,223,1005,224,329,101,1,223,223,7,677,677,224,102,2,223,223,1006,224,344,101,1,223,223,108,677,677,224,1002,223,2,223,1006,224,359,1001,223,1,223,7,677,226,224,1002,223,2,223,1005,224,374,1001,223,1,223,1107,677,226,224,1002,223,2,223,1005,224,389,1001,223,1,223,107,226,677,224,102,2,223,223,1005,224,404,1001,223,1,223,1108,226,677,224,1002,223,2,223,1006,224,419,101,1,223,223,107,226,226,224,102,2,223,223,1005,224,434,1001,223,1,223,108,677,226,224,1002,223,2,223,1006,224,449,101,1,223,223,108,226,226,224,102,2,223,223,1006,224,464,1001,223,1,223,1007,226,226,224,1002,223,2,223,1005,224,479,101,1,223,223,8,677,226,224,1002,223,2,223,1006,224,494,101,1,223,223,1007,226,677,224,102,2,223,223,1006,224,509,101,1,223,223,7,226,677,224,1002,223,2,223,1005,224,524,101,1,223,223,107,677,677,224,102,2,223,223,1005,224,539,101,1,223,223,1008,677,226,224,1002,223,2,223,1005,224,554,1001,223,1,223,1008,226,226,224,1002,223,2,223,1006,224,569,1001,223,1,223,1108,226,226,224,102,2,223,223,1005,224,584,101,1,223,223,1107,226,677,224,1002,223,2,223,1005,224,599,1001,223,1,223,8,226,677,224,1002,223,2,223,1006,224,614,1001,223,1,223,1108,677,226,224,102,2,223,223,1005,224,629,1001,223,1,223,8,226,226,224,1002,223,2,223,1005,224,644,1001,223,1,223,1107,677,677,224,1002,223,2,223,1005,224,659,1001,223,1,223,1007,677,677,224,1002,223,2,223,1005,224,674,101,1,223,223,4,223,99,226];
        let output = IntCode::new(&input).run([1].iter());
        assert_eq!(output, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 6761139]);

        let output = IntCode::new(&input).run([5].iter());
        assert_eq!(output, vec![9217546]);
    }

    #[test]
    fn test_day9() {
        let input = vec![1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1102,3,1,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,904,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1101,0,34,1006,1101,0,689,1022,1102,27,1,1018,1102,1,38,1010,1102,1,31,1012,1101,20,0,1015,1102,1,791,1026,1102,0,1,1020,1101,24,0,1000,1101,0,682,1023,1101,788,0,1027,1101,0,37,1005,1102,21,1,1011,1102,1,28,1002,1101,0,529,1024,1101,39,0,1017,1102,30,1,1013,1101,0,23,1003,1102,524,1,1025,1101,32,0,1007,1102,25,1,1008,1101,29,0,1001,1101,33,0,1016,1101,410,0,1029,1101,419,0,1028,1101,22,0,1014,1102,26,1,1019,1102,1,35,1009,1102,36,1,1004,1102,1,1,1021,109,11,2107,22,-8,63,1005,63,199,4,187,1106,0,203,1001,64,1,64,1002,64,2,64,109,2,21108,40,40,-2,1005,1011,221,4,209,1106,0,225,1001,64,1,64,1002,64,2,64,109,13,21102,41,1,-7,1008,1019,41,63,1005,63,251,4,231,1001,64,1,64,1106,0,251,1002,64,2,64,109,-19,1202,1,1,63,1008,63,26,63,1005,63,271,1105,1,277,4,257,1001,64,1,64,1002,64,2,64,109,7,2101,0,-6,63,1008,63,24,63,1005,63,297,1106,0,303,4,283,1001,64,1,64,1002,64,2,64,109,7,1205,-1,315,1105,1,321,4,309,1001,64,1,64,1002,64,2,64,109,-11,21107,42,41,0,1005,1010,341,1001,64,1,64,1106,0,343,4,327,1002,64,2,64,109,-8,1207,6,24,63,1005,63,363,1001,64,1,64,1106,0,365,4,349,1002,64,2,64,109,11,1206,8,381,1001,64,1,64,1106,0,383,4,371,1002,64,2,64,109,4,1205,4,401,4,389,1001,64,1,64,1105,1,401,1002,64,2,64,109,14,2106,0,-3,4,407,1001,64,1,64,1106,0,419,1002,64,2,64,109,-33,1202,3,1,63,1008,63,29,63,1005,63,445,4,425,1001,64,1,64,1105,1,445,1002,64,2,64,109,-5,2102,1,7,63,1008,63,25,63,1005,63,465,1105,1,471,4,451,1001,64,1,64,1002,64,2,64,109,11,21107,43,44,7,1005,1011,489,4,477,1105,1,493,1001,64,1,64,1002,64,2,64,109,-3,1208,8,35,63,1005,63,511,4,499,1105,1,515,1001,64,1,64,1002,64,2,64,109,25,2105,1,-2,4,521,1106,0,533,1001,64,1,64,1002,64,2,64,109,-8,21108,44,47,-8,1005,1010,549,1106,0,555,4,539,1001,64,1,64,1002,64,2,64,109,-19,1207,7,35,63,1005,63,577,4,561,1001,64,1,64,1106,0,577,1002,64,2,64,109,2,2108,32,0,63,1005,63,597,1001,64,1,64,1106,0,599,4,583,1002,64,2,64,109,13,2101,0,-7,63,1008,63,32,63,1005,63,625,4,605,1001,64,1,64,1105,1,625,1002,64,2,64,109,-13,2107,24,2,63,1005,63,645,1001,64,1,64,1106,0,647,4,631,1002,64,2,64,109,18,21101,45,0,-4,1008,1015,43,63,1005,63,671,1001,64,1,64,1105,1,673,4,653,1002,64,2,64,109,-6,2105,1,10,1001,64,1,64,1105,1,691,4,679,1002,64,2,64,109,1,1208,-6,23,63,1005,63,707,1105,1,713,4,697,1001,64,1,64,1002,64,2,64,109,-2,1206,8,731,4,719,1001,64,1,64,1106,0,731,1002,64,2,64,109,-7,21102,46,1,5,1008,1010,43,63,1005,63,751,1106,0,757,4,737,1001,64,1,64,1002,64,2,64,109,-9,2108,24,4,63,1005,63,779,4,763,1001,64,1,64,1106,0,779,1002,64,2,64,109,38,2106,0,-7,1106,0,797,4,785,1001,64,1,64,1002,64,2,64,109,-27,2102,1,-6,63,1008,63,29,63,1005,63,819,4,803,1105,1,823,1001,64,1,64,1002,64,2,64,109,1,21101,47,0,7,1008,1015,47,63,1005,63,845,4,829,1105,1,849,1001,64,1,64,1002,64,2,64,109,-11,1201,5,0,63,1008,63,31,63,1005,63,869,1106,0,875,4,855,1001,64,1,64,1002,64,2,64,109,5,1201,4,0,63,1008,63,34,63,1005,63,901,4,881,1001,64,1,64,1105,1,901,4,64,99,21102,27,1,1,21101,915,0,0,1105,1,922,21201,1,58905,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21101,0,942,0,1106,0,922,22101,0,1,-1,21201,-2,-3,1,21102,1,957,0,1106,0,922,22201,1,-1,-2,1106,0,968,22102,1,-2,-2,109,-3,2106,0,0];
        let mut program = IntCode::new(&input);
        program.biggen(2000);
        let output = program.run([1].iter());
        assert_eq!(output, vec![3280416268]);

        let mut program = IntCode::new(&input);
        program.biggen(2000);
        let output = program.run([2].iter());
        assert_eq!(output, vec![80210]);
    }

}
