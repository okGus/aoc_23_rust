use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use regex::Regex;
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let all_digits = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let mut sum: i64 = 0;
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            if let Ok(str) = line {
                let mut first_index = str.len();
                let mut last_index: Option<usize> = None;
                let mut first_val = 0;
                let mut last_val = 0;

                for digit in all_digits.keys() {
                    if let Some(index) = str.find(digit) {
                        if index < first_index {
                            first_index = index;
                            first_val = *all_digits.get(digit).unwrap();
                        }
                    }
                    if let Some(index) = str.rfind(digit) {
                        // last_index is an option<usize>
                        // in the case there is no digit found - map_or to handle option - return default true if last_index is none
                        // |last_index| defines a closure with a single parameter - short hand notation for an anonymous function
                        // then it does index > last_index
                        // If last_index is None, return true (because there is no previous last to compare).
                        // If last_index is Some(last), return the result of index > last.
                        if last_index.map_or(true, |last_index| index > last_index) {
                            last_index = Some(index);
                            last_val = *all_digits.get(digit).unwrap();
                        }
                    }
                }

                let num = first_val * 10 + last_val;
                sum += num;
            }
        }
        println!("{}", sum)
    }
}
