mod input;
use day11::{IntCode, IoOperation};

fn main() {

    // part 1

    let mut program = IntCode::new(&input::ORIGINAL);
    program.biggen(10000);
    let mut robot = Robot::new();
    program.run_fn(|op| {
        match op {
            IoOperation::Read => robot.current_color(),
            IoOperation::Write(val) => robot.execute(val),
        }
    });
    println!("{}", robot.panels_painted);

    // part 2

    let mut program = IntCode::new(&input::ORIGINAL);
    program.biggen(10000);
    let mut robot = Robot::new();
    robot.panel[robot.x][robot.y] = Paint::White;
    program.run_fn(|op| {
        match op {
            IoOperation::Read => robot.current_color(),
            IoOperation::Write(val) => robot.execute(val),
        }
    });
    for row in robot.panel.iter() {
        for x in row.iter() {
            if let Paint::White = x {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[derive(Clone, Copy)]
enum Paint {
    Unpainted,
    Black,
    White,
}

struct Robot {
    panel: [[Paint; 100]; 100],
    x: usize,
    y: usize,
    direction: Direction,
    is_reading_color: bool,
    panels_painted: i32,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            panel: [[Paint::Unpainted; 100]; 100],
            x: 50,
            y: 50,
            direction: Direction::Up,
            is_reading_color: true,
            panels_painted: 0,
        }
    }

    fn current_color(&mut self) -> i64 {
        if let Paint::White = self.panel[self.x][self.y] {
            1
        } else {
            0
        }
    }

    fn execute(&mut self, val: i64) -> i64 {
        if self.is_reading_color {
            if let Paint::Unpainted = self.panel[self.x][self.y] {
                self.panels_painted += 1;
            }
            self.panel[self.x][self.y] = if val == 1 {
                Paint::White
            } else {
                Paint::Black
            };
        } else {
            self.direction = if val == 0 {
                Direction::rotate_left(self.direction)
            } else {
                Direction::rotate_right(self.direction)
            };
            match self.direction {
                Direction::Up => self.y += 1,
                Direction::Down => self.y -= 1,
                Direction::Right => self.x += 1,
                Direction::Left => self.x -= 1,
            };
        }
        self.is_reading_color = !self.is_reading_color;
        0
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up, Left, Down, Right
}

impl Direction {
    fn rotate_left(d: Direction) -> Direction {
        match d {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Left => Direction::Down,
        }
    }

    fn rotate_right(d: Direction) -> Direction {
        match d {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
        }
    }
}
