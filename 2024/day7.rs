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

fn check_validity(equation: &[u64]) -> u64{
    let mut goal = equation[0];
    let rest = &equation[1..];
    let mut sum = rest.iter().fold(0, |acc, val| acc + val);

    for operand in rest.iter().rev() {
        if goal == sum {
            break;
        }

        let quotient = goal / operand;
        let residue = goal - quotient * operand;

        if residue == 0 {
            goal = quotient;
        }
        else {
            if goal < *operand {
                break;
            }
            goal -= operand;
        }

        sum -= operand;
    }

    if goal == sum {
        equation[0]
    }
    else {
        0
    }
}

fn main() {
    let lists = env::args().nth(1).as_ref()
                        .map(|filename| read_to_string(filename))
                        .expect("need a file as input")
                        .expect("input can't be read")
                        .lines()
                        .map(|line| line.split(|c| c == ':' || c == ' ')
                                        .filter_map(|chunk| chunk.parse::<u64>().ok())
                                        .collect::<Vec<_>>()
                        )
                        .collect::<Vec<Vec<_>>>();

    let mut ans = 0;
    for list in lists {
        ans += check_validity(&list);
    }

    println!("{}", ans);
}

// build test harness with `rustc --test day7.rs`
#[cfg(test)]
mod test {
    use super::check_validity;

    #[test]
    fn basic() {
        let ok_equation = vec![3267, 81, 40, 27];
        assert_eq!(3267, check_validity(&ok_equation));

        let ok_equation = vec![292, 11, 6, 16, 20];
        assert_eq!(292, check_validity(&ok_equation));

        let bad_equation = vec![83, 17, 5];
        assert_eq!(0, check_validity(&bad_equation));

        let bad_equation = vec![161011, 16, 10, 13];
        assert_eq!(0, check_validity(&bad_equation));
    }

    #[test]
    fn trickier_stuff() {
        let ok_equation = vec![25, 5, 5, 5, 5, 5];
        assert_eq!(25, check_validity(&ok_equation));

        let ok_equation = vec![25, 5, 5];
        assert_eq!(25, check_validity(&ok_equation));

        let bad_equation = vec![3, 17, 5];
        assert_eq!(0, check_validity(&bad_equation));
    }

    #[test]
    fn tricky_ones() {
        let ok_equation = vec![5, 1, 1, 1, 1, 5];
        assert_eq!(5, check_validity(&ok_equation));

        let ok_equation = vec![5, 5, 1, 1, 1, 1];
        assert_eq!(5, check_validity(&ok_equation));

        let ok_equation = vec![5, 1, 1, 1, 1, 1];
        assert_eq!(5, check_validity(&ok_equation));

        let ok_equation = vec![5, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(5, check_validity(&ok_equation));
    }

    #[test]
    fn extra_ones() {
        let ok_equation = vec![3267, 81, 40, 27, 1];
        assert_eq!(3267, check_validity(&ok_equation));

        let ok_equation = vec![3267, 81, 1, 40, 27];
        assert_eq!(3267, check_validity(&ok_equation));

        let ok_equation = vec![25, 5, 1, 1, 5];
        assert_eq!(25, check_validity(&ok_equation));

        let ok_equation = vec![292, 11, 6, 1, 1, 16, 20];
        assert_eq!(292, check_validity(&ok_equation));

        let ok_equation = vec![292, 11, 4, 1, 1, 16, 20];
        assert_eq!(292, check_validity(&ok_equation));
    }

    #[test]
    fn divisibility_check_fails() {
        let ok_equation = vec![42, 6, 6, 6];
        assert_eq!(42, check_validity(&ok_equation));

        let ok_equation = vec![20, 2, 8, 4];
        assert_eq!(20, check_validity(&ok_equation));
    }
}
