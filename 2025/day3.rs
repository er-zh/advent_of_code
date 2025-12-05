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
                .map(|line| line.bytes()
                                .map(|b| b - 48)
                                .fold((0, 0), |(mut tens, mut ones), digit| {
                                    if ones > tens {
                                        tens = ones;
                                        ones = 0;
                                    }
                                    if digit > ones {
                                        ones = digit;
                                    }

                                    (tens, ones)
                                })
                )
                .map(|(tens, ones)| tens * 10 + ones)
                .fold(0u32, |acc, joltage| acc + joltage as u32);

    eprintln!("{:?}", ans);
}

