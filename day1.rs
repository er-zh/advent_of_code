use std::env;
use std::fs::File;
use std::io::Read;
use std::io;

fn read_to_string(filename: &str) -> io::Result<String>
{
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

fn main()
{
    let name = env::args().nth(1).expect("need a file name");

    let text = read_to_string(&name).expect("bad file");

    let mut aggregate = 0u32;

    for line in text.lines()
    {
        // solution for part 1
        /*
        let filter: Vec<char> = line.chars().filter(|ch| ch.is_digit(10)).collect();
        aggregate += filter[0].to_digit(10).unwrap() * 10u32 + &filter.last().and_then(|x| x.to_digit(10)).unwrap();
        */

        // solution for part 2
        let mut idx = 0usize;
        let substr = |i, j| { line.get(i..j).unwrap_or("") };
        let make_digits = |ch| {
            let res = match ch {
                'o' => {
                    if substr(idx, idx+3) == "one" {1} else {0}
                },
                't' => {
                    if substr(idx, idx+3) == "two" {2}
                    else if substr(idx, idx+5) == "three" {3}
                    else {0}
                },
                'f' => {
                    if substr(idx, idx+4) == "four" {4}
                    else if substr(idx, idx+4) == "five" {5}
                    else {0}
                },
                's' => {
                    if substr(idx, idx+3) == "six" {6}
                    else if substr(idx, idx+5) == "seven" {7}
                    else {0}
                },
                'e' => {
                    if substr(idx, idx+5) == "eight" {8} else {0}
                },
                'n' => {
                    if substr(idx, idx+4) == "nine" {9} else {0}
                },
                _ => {ch.to_digit(10).unwrap_or(0)}
            };
            idx += 1;
            res
        };
        let uints: Vec<u32> = line.chars().map(make_digits).collect();
        // why do I need a borrow to 0u32 here?... (aside from the fact the compiler tells me to)
        let filter: Vec<u32> = uints.into_iter().filter(|i| i>&0u32).collect();
        aggregate += filter[0] * 10u32 + filter.last().unwrap();
    }

    println!("{}", aggregate);
}

