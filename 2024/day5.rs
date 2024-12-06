use std::env;
use std::fs::File;
use std::io::Read;
use std::io;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

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

    let mut lines_iter = text.lines();

    let mut rules: HashMap<_, HashSet<_>> = HashMap::new();
    for line in lines_iter.by_ref() {
        let mut parts = line.split('|');
        let first = parts.next().map(|text| text.parse::<u32>());
        let second = parts.next().map(|text| text.parse::<u32>());

        if let (Some(Ok(first)), Some(Ok(second))) = (first, second) {
            rules.entry(first).and_modify(|set| {set.insert(second);}).or_insert(HashSet::from([second]));
        }
        else {
            break;
        }
    }

    let comp_pages = |first: &u32, second: &u32| {
        if let Some(precedence) = rules.get(first) {
            if precedence.contains(second) {
                return Ordering::Less;
            }
        }
        else if let Some(precedence) = rules.get(second) {
            if precedence.contains(first) {
                return Ordering::Greater;
            }
        }

        return Ordering::Equal;
    };

    let lists = lines_iter.map(
        |list_txt| {
            list_txt.split(',')
                .filter_map(|text| text.parse::<u32>().ok())
                .collect::<Vec<_>>()
        }
    )
    .collect::<Vec<Vec<_>>>();

    let mut ans = 0;
    let mut ans2 = 0;
    for list in lists {
        let mut check = list.clone();
        check.sort_by(comp_pages);

        if &list == &check {
            ans += list[list.len()/2];
        }
        else {
            ans2 += check[check.len()/2];
        }
    }

    println!("{} {}", ans, ans2);
}
