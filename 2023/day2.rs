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

    /* soln for part 1
    let reds = 12u32;
    let greens = 13u32;
    let blues = 14u32;
    */

    let ans = text
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split(' ').collect::<Vec<&str>>()) // split into tokens
        .fold(0u32, |acc, vec| {
            let val = vec[1].replace(":","").parse::<u32>().unwrap();
            let rest = vec[2..].chunks(2);

            /* soln for part 1
            for pair in rest {
                let color = pair[1].replace(",", "").replace(";","");
                let amt = pair[0].parse::<u32>().unwrap();
                match color.as_str() {
                    "red" => if amt > reds { return acc },
                    "blue" => if amt > blues { return acc },
                    "green" => if amt > greens { return acc },
                    _ => panic!("unexpected input for game {}", val)
                };
            }

            acc + val
            */

            // soln for part 2
            let mut red_needed = 0u32;
            let mut blue_needed = 0u32;
            let mut green_needed = 0u32;
            for pair in rest {
                let color = pair[1].replace(",", "").replace(";","");
                let amt = pair[0].parse::<u32>().unwrap();
                match color.as_str() {
                    "red" => if amt > red_needed { red_needed = amt; },
                    "blue" => if amt > blue_needed { blue_needed = amt; },
                    "green" => if amt > green_needed { green_needed = amt; },
                    _ => panic!("unexpected input for game {}", val)
                };
            }

            acc + red_needed * blue_needed * green_needed
        });
    println!("{}", ans);
}
