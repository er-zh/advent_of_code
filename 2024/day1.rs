use std::env;
use std::fs::File;
use std::io::Read;
use std::io;
use std::collections::HashMap;

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

    let (mut left, mut right): (Vec<_>, Vec<_>) = text.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let first = parts.next().unwrap().parse::<u32>().unwrap();
            let second = parts.next().unwrap().parse::<u32>().unwrap();
            (first, second)
        })
        .unzip();

    /* soln for part 1
    left.sort();
    right.sort();

    let ans = left.iter()
                .zip(right.iter())
                .map(|(first, second)| {
                    if first > second {
                        (second, first)
                    }
                    else {
                        (first, second)
                    }
                })
                .fold(0, |acc, (first, second)| {
                    acc + second - first
                });
    */

    let mut lmap = HashMap::new();
    let mut rmap = HashMap::new();

    for (left, right) in left.iter().zip(right.iter()) {
        lmap.entry(left).and_modify(|counter| *counter += 1).or_insert(1);
        rmap.entry(right).and_modify(|counter| *counter += 1).or_insert(1);
    }

    let mut ans = 0;

    for (key, l_amt) in lmap.drain() {
        ans += key * l_amt * rmap.get(key).map_or(0, |&res| res);
    }

    println!("{}", ans);
}
