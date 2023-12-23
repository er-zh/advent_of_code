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

    let numbers = text.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split(' ').filter(|chunk| !chunk.is_empty()).collect::<Vec<&str>>())
        .map(|vec| vec![vec[1..].join("")]) // comment out this line for part 1's soln
        .map(|vec| vec.iter().filter_map(|word| word.parse::<u64>().ok()).collect::<Vec<u64>>())
        .collect::<Vec<Vec<u64>>>();

    let ans = numbers[0].iter().zip(numbers[1].iter())
        .fold(1u64, |acc, pair| {
            let (time, dist) = pair;

            for i in 0..=*time/2
            {
                if (*time - i) * i > *dist
                {
                    return acc * (*time + 1 - 2* i);
                }
            }
            0
        });

    println!("{:?}", ans);
}

