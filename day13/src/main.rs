use std::{thread, time};
use day13::{IntCode, IoOperation};
use ncurses::*;

mod input;

const WIDTH: usize = 50;
const HEIGHT: usize = 25;

fn main() {
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    clear();

    part_1();
    getch();
    part_2();

    getch();
    endwin();
}

fn part_2() -> i64 {
    let memory = &mut input::ORIGINAL.clone();
    memory[0] = 2;

    let (mut x, mut y, mut count) = (0, 0, 0);

    let mut program = IntCode::new(memory);
    program.biggen(10_000);

    let mut joystick = 0;
    let mut score = 0;
    let mut ballpos = 0;
    let mut padpos = 0;
    program.run_fn(|op| match op {
        IoOperation::Read => {
            // uncomment this section to play yourself
            // match getch() {
            //     108 => 1,
            //     115 => -1,
            //     _ => 0,
            // }
            if padpos > ballpos {
                -1
            } else if ballpos > padpos {
                1
            } else {
                0
            }
        }
        IoOperation::Write(val) => {
            count += 1;
            match count {
                1 => x = val,
                2 => y = val,
                _ => {
                    if x == -1 && y == 0 {
                        score = val;
                        mvprintw(0, 0, &format!("score: {} ----------------------\n", score));
                        refresh();
                        // thread::sleep(time::Duration::from_millis(10));
                    } else {
                        if val == 3 {
                            padpos = x;
                        } else if val == 4 {
                            ballpos = x;
                        }
                        mvprintw(
                            y as i32,
                            x as i32,
                            match val {
                                0 => " ",
                                1 => "|",
                                2 => "X",
                                3 => "X",
                                4 => "O",
                                _ => "?",
                            },
                        );
                    }
                    count = 0;
                }
            };
            0
        }
    });
    score
}

fn part_1() {
    let (mut x, mut y, mut count) = (0, 0, 0);
    let mut panel = [[0; WIDTH]; HEIGHT];
    let mut program = IntCode::new(&input::ORIGINAL);
    program.biggen(10_000);
    program.run_fn(|op| match op {
        IoOperation::Read => 1,
        IoOperation::Write(val) => {
            count += 1;
            match count {
                1 => x = val,
                2 => y = val,
                _ => {
                    panel[y as usize][x as usize] = val;
                    count = 0;
                }
            };
            0
        }
    });
    mvprintw(0, 0, 
        &format!(
            "number of bricks: {}",
            panel
                .iter()
                .map(|x| { x.iter() })
                .flatten()
                .filter(|x| { **x == 2 })
                .count()
        )
    );
    refresh();
}
