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

    let mut aggregate: u32 = 0;

    for line in text.lines()
    {
        // solution for part 1
        /*
        let filter: Vec<char> = line.chars().filter(|ch| ch.is_digit(10)).collect();
        aggregate += filter[0].to_digit(10).unwrap() * 10u32 + &filter.last().unwrap().to_digit(10).unwrap();
        */
    }

    println!("{}", aggregate);
}

