use std::env;
use std::fs::File;
use std::io::Read;
use std::io;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

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

    let num_rows = text.lines().count();
    let num_cols = text.lines().nth(0).expect("empty file").chars().count();

    let mut collect: u32 = 0;
    let mut stop_collect = false;
    let mut trailing: Vec<usize> = vec![];

    let mut sym_indices: Vec<(usize, char)> = vec![];
    let mut num_map = HashMap::new();

    for (idx, letter) in text.replace('\n', "").char_indices()
    {
        match letter.to_digit(10)
        {
            Some(n) =>
            {
                collect = collect * 10 + n;
                trailing.push(idx);

                // make sure that you don't read past the end of the line
                if idx - num_cols * (idx/num_cols) == num_cols - 1
                {
                    stop_collect = true;
                }
            },
            None =>
            {
                if collect > 0
                {
                    stop_collect = true;
                }
                if letter != '.'
                {
                    sym_indices.push((idx, letter));
                }
            }
        }

        if stop_collect
        {
            let val = Rc::new(RefCell::new(Some(collect)));
            while let Some(i) = trailing.pop()
            {
                num_map.insert(i, val.clone());
            }
            collect = 0;
            stop_collect = false;
        }
    }

    let mut ans: u32 = 0;
    // soln for part 1
    /*
    for (idx, _) in &sym_indices
    {
        let row = idx / num_cols;
        let rows = [if row > 0 {Some(row-1)} else {None}, Some(row), if row < num_rows-1 {Some(row+1)} else {None}];

        let col = idx - num_cols * row;
        let cols = [if col > 0 {Some(col-1)} else {None}, Some(col), if col < num_cols-1 {Some(col+1)} else {None}];

        for i in rows.iter().filter(|v| v.is_some()).map(|v| v.unwrap())
        {
            for j in cols.iter().filter(|v| v.is_some()).map(|v| v.unwrap())
            {
                match num_map.get(&(i * num_cols + j))
                {
                    Some(val) => {
                        let refc = Rc::try_unwrap(val.into()).unwrap_or_else(|_| panic!("failed to unwrap Rc"));
                        let inside = refc.replace(None);
                        match inside {
                            Some(num) => ans += num,
                            None => ()
                        }
                    },
                    None => ()
                }
            }
        }
    }
    */

    // this is roughly the same as part 1, just with a few extra checks to see if something is a
    // gear or not
    for (idx, letter) in &sym_indices
    {
        if *letter != '*' { continue; }

        let mut part_nums: Vec<u32> = vec![];

        let row = idx / num_cols;
        let rows = [if row > 0 {Some(row-1)} else {None}, Some(row), if row < num_rows-1 {Some(row+1)} else {None}];

        let col = idx - num_cols * row;
        let cols = [if col > 0 {Some(col-1)} else {None}, Some(col), if col < num_cols-1 {Some(col+1)} else {None}];

        for i in rows.iter().filter(|v| v.is_some()).map(|v| v.unwrap())
        {
            for j in cols.iter().filter(|v| v.is_some()).map(|v| v.unwrap())
            {
                match num_map.get(&(i * num_cols + j))
                {
                    Some(val) => {
                        let refc = Rc::try_unwrap(val.into()).unwrap_or_else(|_| panic!("failed to unwrap Rc"));
                        let inside = refc.replace(None);
                        match inside {
                            Some(num) => part_nums.push(num),
                            None => ()
                        }
                    },
                    None => ()
                }
            }
        }

        if part_nums.len() == 2
        {
            ans += part_nums[0] * part_nums[1];
        }
    }

    println!("{}", ans);
}

