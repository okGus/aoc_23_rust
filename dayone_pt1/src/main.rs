use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut sum: i64 = 0;
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            if let Ok(str) = line {
                let mut n_str = String::new();
                for s in str.chars() {
                    if s.is_numeric() {
                        n_str += &String::from(s);
                        break;
                    }
                }
                let rev_str = str.chars().rev().collect::<String>();
                for s in rev_str.chars() {
                    if s.is_numeric() {
                        n_str += &String::from(s);
                        break;
                    }
                }
                println!("{}", n_str);
                let l: i64 = n_str.trim().parse().unwrap_or(0);
                sum += l;
            }
        }
        println!("{}", sum)
    }
}
