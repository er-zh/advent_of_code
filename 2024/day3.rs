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

    /* part 1
    let ans = text.split("mul(")
        .flat_map(|chunk| chunk.split(")").next())
        .map(|chunk| chunk.split(",").collect::<Vec<_>>())
        .filter(|res| res.len() == 2)
        .filter_map(|params| params.iter()
                            .map(|val| val.parse::<u32>())
                            .collect::<Result<Vec<_>, _>>()
                            .ok()
        )
        .map(|operands| operands.iter().fold(1, |acc, op| acc * op))
        .fold(0, |acc, product| acc + product);
    */

    // part 2
    let ans = text.split("do()")
        .flat_map(|chunk| chunk.split("don't()").next())
        .flat_map(|chunk| chunk.split("mul("))
        .flat_map(|chunk| chunk.split(")").next())
        .map(|chunk| chunk.split(",").collect::<Vec<_>>())
        .filter(|res| res.len() == 2)
        .filter_map(|params| params.iter()
                            .map(|val| val.parse::<u32>())
                            .collect::<Result<Vec<_>, _>>()
                            .ok()
        )
        .map(|operands| operands.iter().fold(1, |acc, op| acc * op))
        .fold(0, |acc, product| acc + product);

    println!("{:?}", ans);
}
