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
    let mut power_set = 0;
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            if let Ok(input) = line {
                let mut sum_set = 1;
                let mut colors = HashMap::from([
                    ("red", -1),
                    ("green", -1),
                    ("blue", -1),
                ]);
                // stuff
                let mut _skip_game = false;
                let re = Regex::new(r"\d+ (blue|red|green);*").unwrap();
                let ren = Regex::new(r"Game \d+").unwrap();
                let caps: Vec<_> = re.find_iter(&input).collect();
                let _game = ren.find(&input).unwrap().as_str();
                
                //println!("{}", game);
                //println!("colors {:?}", colors);
                
                for reg_match in &caps {
                    let re_color = Regex::new(r"blue|red|green").unwrap();
                    let re_num = Regex::new(r"\d+").unwrap();
                    let color = re_color.find(reg_match.as_str()).unwrap().as_str();
                    let str_num = re_num.find(reg_match.as_str()).unwrap().as_str();
                    let num = str_num.parse::<i32>().unwrap();
                    
                    //println!("**** {} {}", str_num, color);

                    if *colors.get(color).unwrap() < num {
                        *colors.get_mut(color).unwrap() = num;
                    }
                    //println!("updated {:?}", colors);
                }

                for num in colors.values() {
                    sum_set *= num;
                }
                power_set += sum_set;
            }
        }
    }
    println!("{}", power_set);
}
