use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let colors = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    let mut sum = 0;
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            if let Ok(input) = line {
                // stuff
                let mut skip_game = false;
                let re = Regex::new(r"\d+ (blue|red|green);*").unwrap();
                let ren = Regex::new(r"Game \d+").unwrap();
                let caps: Vec<_> = re.find_iter(&input).collect();
                let game = ren.find(&input).unwrap().as_str();
                //println!("{:?}", caps);
                //println!("{:?}", game);

                for cube in &caps {
                    let re_color = Regex::new(r"blue|red|green").unwrap();
                    let re_num = Regex::new(r"\d+").unwrap();
                    let color = re_color.find(cube.as_str()).unwrap().as_str();
                    let num = re_num.find(cube.as_str()).unwrap().as_str();

                    if *colors.get(color).unwrap() < num.parse::<i32>().unwrap() {
                        skip_game = true;
                        break;
                    }
                }
                if skip_game {
                    continue;
                }
                let reg_num = Regex::new(r"\d+").unwrap();
                let game_num = reg_num.find(&game).unwrap().as_str();
                let n: i32 = game_num.trim().parse().unwrap();
                sum += n;
            }
        }
    }
    println!("{}", sum);
}
