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

    let lines_iter = text.lines()
                        .map(|line| {
                            line.split_whitespace()
                                .map(|lexeme| {
                                        lexeme.parse::<i32>().unwrap()
                                })
                                .collect::<Vec<i32>>()
                        })
                        ;

    let mut ans = 0;
    let mut ans_damped = 0;
    for report in lines_iter {

        let safety_check = |levels: &Vec<i32>| {
            let deltas = levels.iter().zip(levels.iter().skip(1))
                            .map(|(start, end)| {
                                end - start
                            });
            let mut inc = 0;
            let mut dec = 0;
            let mut total = 0;

            for diff in deltas {
                if diff < 0 && -4 < diff {
                    inc += 1;
                }
                else if 0 < diff && diff < 4 {
                    dec += 1;
                }

                total += 1;
            }

            total == inc || total == dec
        };


        if safety_check(&report) {
            ans += 1;
            ans_damped += 1;
        }
        else {
            for i in 0..report.len() {
                let mut new_deltas = report.clone();
                new_deltas.remove(i);

                if safety_check(&new_deltas) {
                    ans_damped += 1;
                    break;
                }
            }
        }
    }

    println!("{} {}", ans, ans_damped);
}
