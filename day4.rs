use std::env;
use std::fs::File;
use std::io::Read;
use std::io;

use std::collections::HashSet;

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

    let winnings = text.lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.replace(":", "|"))
            .map(|line| line.split('|').map(str::to_owned).collect::<Vec<String>>())
            /* soln for part 1
            .fold(0u32, |acc, vec| {
                let winners = vec[1].split(' ').filter_map(|val| val.parse::<u32>().ok()).collect::<HashSet<_>>();
                let numbers = vec[2].split(' ').filter_map(|val| val.parse::<u32>().ok()).collect::<Vec<_>>();

                let mut points: u32 = 0;
                for i in &numbers
                {
                    if winners.contains(&i)
                    {
                        if points == 0 { points += 1; }
                        else { points *= 2; }
                    }
                }

                acc + points
            }); can just print the value returned here*/
            .map(|vec| {
                let winners = vec[1].split(' ').filter_map(|val| val.parse::<u32>().ok()).collect::<HashSet<_>>();
                let numbers = vec[2].split(' ').filter_map(|val| val.parse::<u32>().ok()).collect::<Vec<_>>();

                let mut cards_won: usize = 0;
                for i in &numbers
                {
                    if winners.contains(&i)
                    {
                        cards_won += 1;
                    }
                }

                cards_won
            }).collect::<Vec<usize>>();

    let mut cards = vec![1; winnings.len()]; // there is at least one of each scratch card

    for i in 0..winnings.len()
    {
        for j in 1..=winnings[i]
        {
            cards[i+j] += cards[i];
        }
    }

    let ans: usize = cards.iter().sum();

    println!("{}", ans);
}

