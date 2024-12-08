use std::env;
use std::fs;

const OOB: u8 = 10u8;
const HASHTAG: u8 = 35u8;
const START: u8 = 94u8;
const VISIT: u8 = 64u8;

enum Direction {
    North,
    East,
    South,
    West
}

fn main() {
    let mut bytes = env::args().nth(1).as_ref()
                        .map(|filename| fs::read(filename))
                        .expect("need a file as input")
                        .expect("input can't be read");

    let mut row_len = 0;
    let mut start_location = 0;
    for i in 0..bytes.len() {
        if bytes[i] == OOB && row_len == 0 {
            row_len = i;
        }

        if bytes[i] == START {
            start_location = i;
        }

        if row_len != 0 && start_location != 0 {
            break;
        }
    }

    let col_shift_amt = row_len+1; // to account for newlines

    let mut grid = vec![OOB; col_shift_amt];
    grid.append(&mut bytes);
    grid.append(&mut vec![OOB; col_shift_amt]);

    println!("{} {}", row_len, start_location);

    let mut pos = start_location + col_shift_amt;

    let mut heading = Direction::North;
    let update = |heading: &Direction, pos| match heading {
            Direction::North => pos - col_shift_amt,
            Direction::East => pos + 1,
            Direction::South => pos + col_shift_amt,
            Direction::West => pos - 1
    };

    while grid[pos] != OOB {
        grid[pos] = VISIT;

        let next_pos = update(&heading, pos);
        let ahead = grid[next_pos];

        if ahead == HASHTAG {
            heading = match heading {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North
            };
        }
        else {
            pos = next_pos;
        }
    }

    println!("{:?}", grid);

    let ans = grid.iter().fold(0, |acc, &val| if val == VISIT { acc + 1 } else {acc});

    println!("{}", ans);
}

