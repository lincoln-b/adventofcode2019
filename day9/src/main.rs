mod input;
use day9::IntCode;

fn main() {
    // part 1
    let mut computer = IntCode::new(&input::ORIGINAL);
    computer.biggen(2000);
    computer.run([1].iter());

    // part 2
    let mut computer = IntCode::new(&input::ORIGINAL);
    computer.biggen(2000);
    computer.run([2].iter());
}
