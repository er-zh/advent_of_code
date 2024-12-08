use std::env;
use std::fs;
use std::collections::{HashMap, HashSet};

const OOB: u8 = 10u8;
const EMPTY: u8 = 46u8;

fn main() {
    let bytes = env::args().nth(1).as_ref()
                        .map(|filename| fs::read(filename))
                        .expect("need a file as input")
                        .expect("input can't be read");

    let mut antenna_pos: HashMap<_, Vec<_>> = HashMap::new();
    let mut row_len = 0;
    for i in 0..bytes.len() {
        if bytes[i] == OOB && row_len == 0 {
            row_len = i;
        }

        if bytes[i] != OOB && bytes[i] != EMPTY {
            antenna_pos.entry(bytes[i]).and_modify(|list| list.push(i)).or_insert(vec![i]);
        }
    }

    let true_row_len = row_len + 1;
    let mut antinodes = HashSet::new();

    for (key, locations) in antenna_pos.iter() {
        for i in 0..locations.len() {
            for j in i+1..locations.len() {
                let node1 = locations[i];
                let node2 = locations[j];

                let row1 = node1 / true_row_len;
                let row2 = node2 / true_row_len;

                let x1 = node1 - row1 * true_row_len;
                let x2 = node2 - row2 * true_row_len;

                let difference = node2 - node1;

                if node1 >= difference {
                    let antinode1 = node1 - difference;
                    let ax1 = antinode1 - (antinode1 / true_row_len) * true_row_len;
                    if bytes[antinode1] != OOB {
                        if !((x1 < x2 && x1 <= ax1) || (x2 < x1 && ax1 <= x1)) {
                            antinodes.insert(antinode1);
                        }
                    }
                }

                let antinode2 = node2 + difference;
                let ax2 = antinode2 - (antinode2 / true_row_len) * true_row_len;
                if antinode2 < bytes.len() && bytes[antinode2] != OOB {
                    if !((x1 < x2 && ax2 <= x2) || (x2 < x1 && x2 <= ax2)) {
                        antinodes.insert(antinode2);
                    }
                }
            }
        }
    }

    println!("{}", antinodes.len());
}
