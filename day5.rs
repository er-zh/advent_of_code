use std::env;
use std::fs::File;
use std::io::Read;
use std::io;

use std::boxed::Box;

#[derive(Debug)]
pub struct RangeBst
{
    root: Option<Box<Node>>
}

#[derive(Debug,Clone)]
pub struct Node
{
    source: (u64, u64),
    dest: u64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl RangeBst
{
    pub fn new() -> Self
    {
        let root = None;
        Self{ root }
    }

    pub fn map_dest(&self, src: u64) -> u64
    {
        Self::map(src, &self.root)
    }

    fn map(val: u64, current_node: &Option<Box<Node>>) -> u64
    {
        match current_node
        {
            Some(node) =>
            {
                if val < node.source.0
                {
                    Self::map(val, &node.left)
                }
                else if val > node.source.1
                {
                    Self::map(val, &node.right)
                }
                else
                {
                    val - node.source.0 + node.dest
                }
            },
            None => val,
        }
    }

    pub fn insert(&mut self, source_range: (u64,u64), dest_start: u64)
    {
        let new_node = Box::new(Node {
            source: source_range,
            dest: dest_start,
            left: None,
            right: None
        });

        Self::insert_node(new_node, &mut self.root);
    }

    fn insert_node(node: Box<Node>, current_node: &mut Option<Box<Node>>)
    {
        match current_node
        {
            None => *current_node = Some(node),
            Some(current) => {
                let src = node.source;
                let curr_src = current.source;

                let cmp = Self::compare_ranges(src, curr_src);

                if cmp == -1
                {
                    Self::insert_node(node, &mut current.left);
                }
                else if cmp == 1
                {
                    Self::insert_node(node, &mut current.right);
                }
                else
                {
                    panic!("overlapping ranges given, the ranges are assumed to be disjunct");
                }
            }
        }
    }

    fn compare_ranges(range1: (u64, u64), range2: (u64, u64)) -> i32
    {
        if range1.0 > range2.1 {
            1
        }
        else if range1.1 < range2.0 {
            -1
        }
        else {
            0
        }
    }
}

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

    let mut seeds: Vec<u64> = vec![];
    let mut maps: Vec<RangeBst> = vec![];
    for line in text.lines()
    {
        if line == ""
        {
            continue;
        }

        let tokens = line.split(' ').collect::<Vec<&str>>();

        if tokens.iter().any(|token| *token == "seeds:")
        {
            seeds = tokens[1..].iter().filter_map(|num_str| num_str.parse::<u64>().ok()).collect::<Vec<u64>>();
        }
        else if tokens.iter().any(|token| *token == "map:")
        {
            maps.push(RangeBst::new());
        }
        else
        {
            if let [dest, src, size] = tokens.iter().filter_map(|num_str| num_str.parse::<u64>().ok()).collect::<Vec<u64>>()[..]
            {
                match maps.last_mut()
                {
                    Some(map) => map.insert((src, src+size-1), dest),
                    None => panic!("unexpected input, trying to insert range before any maps created")
                }
            }
            else
            {
                panic!("unexpected input pattern for specifying a map: {:?}", tokens);
            }
        }
    }

    // soln to part 1
    //let ans = seeds.iter().map(|seed| maps.iter().fold(*seed, |acc, map| map.map_dest(acc))).min();

    // unga bunga brute force soln for part 2
    let ans = seeds.chunks(2).map(|range| {
        let range_iter = range[0]..range[0]+range[1]-1;
        range_iter.map(|seed| maps.iter().fold(seed, |acc, map| map.map_dest(acc))).min()
    }).min();

    println!("{:?}", ans);
}
