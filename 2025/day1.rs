use std::env;
use std::fs::File;
use std::io::Read;
use std::io;

fn read_to_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

fn main() {
    let text = env::args().nth(1).as_ref()
                        .map(|filename| read_to_string(filename))
                        .expect("need a file as input")
                        .expect("input can't be read");

    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut dial = 50;
    for line in text.lines() {
        let mut iter = line.chars();
        let c = iter.next().unwrap();
        let amt = iter.collect::<String>().parse::<i32>().unwrap();
        let full_turns = amt / 100;
        let effective_turn = amt - (full_turns * 100);

        ans2 += full_turns;

        let new_val = if c == 'L' {
            dial - effective_turn
        }
        else {
            dial + effective_turn
        };

        if new_val < 0 {
            if dial != 0 {
                ans2 += 1;
            }
            dial = new_val + 100;
        }
        else if new_val > 99 {
            if new_val != 100 {
                ans2 += 1;
            }
            dial = new_val - 100;
        }
        else {
            dial = new_val;
        }

        if dial == 0 {
            ans1 += 1;
            ans2 += 1;
        }
    }

    println!("{} {}", ans1, ans2);
}
