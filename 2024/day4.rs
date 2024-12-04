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

    let mut ans = 0;

    let chars = text.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();
    let height = chars.len();
    let width = chars[0].len();

    for i in 1..height-1 {
        for j in 1..width-1 {
            let char = chars[i][j];

            if char == 'A' {
                let diag = (chars[i-1][j-1], chars[i+1][j+1]);
                let antidiag = (chars[i-1][j+1], chars[i+1][j-1]);

                let mut good = ((diag.0 == 'M') ^ (diag.1 == 'M')) && ((diag.0 == 'S') ^ (diag.1 == 'S'));
                good &= ((antidiag.0 == 'M') ^ (antidiag.1 == 'M')) && ((antidiag.0 == 'S') ^ (antidiag.1 == 'S'));

                if good {
                    ans += 1;
                }
            }
        }
    }

    /* part 1 soln
    let mut columns: Vec<Vec<_>> = vec![];
    let mut diagonals: Vec<Vec<_>> = vec![];
    let mut antidiagonals: Vec<Vec<_>> = vec![];
    let mut antioffset = 0;

    let mut j = 0;
    for line in text.lines() {

        ans += line.match_indices("XMAS").collect::<Vec<_>>().len();
        ans += line.match_indices("SAMX").collect::<Vec<_>>().len();

        for (i, c) in line.chars().enumerate() {
            if let Some(col) = columns.get_mut(i) {
                col.push(c);
            }
            else {
                columns.push(vec![c]);
            }

            if let Some(diag) = diagonals.get_mut(i+j) {
                diag.push(c);
            }
            else {
                diagonals.push(vec![c]);
            }

            let mut temp_idx = 0;
            if i < j {
                temp_idx += antioffset + j - i;
            }
            else {
                temp_idx = i - j;
            }

            if let Some(adiag) = antidiagonals.get_mut(temp_idx) {
                adiag.push(c);
            }
            else {
                antidiagonals.push(vec![c]);
            }
        }

        // true for the first iter then not after
        if antioffset == 0 {
            antioffset = antidiagonals.len() - 1;
        }

        j += 1;
    }

    for col in columns {
        let col: String = col.into_iter().collect();

        ans += col.match_indices("XMAS").collect::<Vec<_>>().len();
        ans += col.match_indices("SAMX").collect::<Vec<_>>().len();
    }

    for diag in diagonals {
        let diag: String = diag.into_iter().collect();

        ans += diag.match_indices("XMAS").collect::<Vec<_>>().len();
        ans += diag.match_indices("SAMX").collect::<Vec<_>>().len();

    }

    for adiag in antidiagonals {
        let adiag: String = adiag.into_iter().collect();

        ans += adiag.match_indices("XMAS").collect::<Vec<_>>().len();
        ans += adiag.match_indices("SAMX").collect::<Vec<_>>().len();
    }
    */

    println!("{}", ans);
}
