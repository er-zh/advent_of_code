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
    let text = env::args()
                    .nth(1).as_ref()
                    .map(|filename| read_to_string(filename))
                    .expect("need a file as input")
                    .expect("input can't be read");

    let ans = text.lines()
                .flat_map(|line| line.split(','))
                .map(|range| {
                    let mut pair = range.split('-');
                    (pair.next().unwrap().parse::<u64>().unwrap(), pair.next().unwrap().parse::<u64>().unwrap())
                })
                .map(|(start, stop)| {
                    let mut place = 1;
                    let mut mult = 1;
                    let mut step = true;
                    while start > place {
                        place *= 10;
                        if step {
                            mult *= 10;
                        }
                        step = !step;
                    }

                    if step {
                        (start / mult, mult, start, stop)
                    }
                    else {
                        (place / mult, mult, start, stop)
                    }
                })
                .fold(0u64, |mut acc, (mut step, mut mult, start, stop)| {
                    println!("{} {} [{}, {}]", step, mult, start, stop);
                    if mult == 1 {
                        mult = 10;
                    }

                    loop {
                        let val = step * mult + step;
                        if stop < val {
                            break;
                        }
                        if start <= val {
                            acc += val;
                            println!("{}", val);
                        }
                        step += 1;
                        if step >= mult {
                            mult *= 10;
                        }
                    }

                    acc
                });

    println!("{:?}", ans);
}
