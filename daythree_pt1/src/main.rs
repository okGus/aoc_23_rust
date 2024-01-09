use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn check(schem: &Vec<Vec<char>>, spec_char: &HashSet<&str>, r: i32, c: i32, mut part_sum: i32, mut same: String, mut sleft: String, mut sright: String) -> i32 {
    let row = r as usize;
    let col = c as usize;

    if r < 0 || r > schem.len() as i32 {
        return part_sum;
    }

    if !spec_char.contains(&schem[row][col].to_string().as_str()) {
        same = schem[row][col].to_string();
        let mut l = col-1;
        let mut r = col+1;
        loop {
            if !spec_char.contains(&schem[row][l].to_string().as_str()) {
                same = schem[row][l].to_string() + &same;
                l -= 1;
            }
            if !spec_char.contains(&schem[row][r].to_string().as_str()) {
                same.push_str(&String::from(schem[row][r].to_string().as_str()));
                r += 1;
            }
            if spec_char.contains(&schem[row][l].to_string().as_str()) && spec_char.contains(&schem[row][r].to_string().as_str()) {
                break;
            }
        }
    }

    if !spec_char.contains(&schem[row][col-1].to_string().as_str()) {
        sleft = String::from(schem[row][col-1]);
        let mut l = (col-1)-1;
        let mut r = (col-1)+1;
        loop {
            if !spec_char.contains(&schem[row][l].to_string().as_str()) {
                sleft = schem[row][l].to_string() + &sleft;
                l -= 1;
            }
            if !spec_char.contains(&schem[row][r].to_string().as_str()) {
                sleft.push_str(&String::from(schem[row][r]));
                r += 1;
            }
            if spec_char.contains(&schem[row][l].to_string().as_str()) && spec_char.contains(&schem[row][r].to_string().as_str()) {
                break;
            }
        }
    }

    if !spec_char.contains(&schem[row][col+1].to_string().as_str()) {
        sright = String::from(schem[row][col+1]);
        let mut l = (col+1)-1;
        let mut r = (col+1)+1;
        loop {
            if !spec_char.contains(&schem[row][l].to_string().as_str()) {
                sright = schem[row][l].to_string() + &sright;
                l -= 1;
            }
            if !spec_char.contains(&schem[row][r].to_string().as_str()) {
                sright.push_str(&String::from(schem[row][r]));
                r += 1;
            }
            if spec_char.contains(&schem[row][l].to_string().as_str()) && spec_char.contains(&schem[row][r].to_string().as_str()) {
                break;
            }
        }
    }

    // if all are same
    if sleft == same && same == sright {
        part_sum += sleft.parse::<i32>().unwrap();
    }

    if spec_char.contains(sleft.as_str()) {
        if !spec_char.contains(same.as_str()) && same == sright {
            part_sum += same.parse::<i32>().unwrap();
        }
        else if !spec_char.contains(same.as_str()) && !spec_char.contains(sright.as_str()){
            part_sum += same.parse::<i32>().unwrap();
            part_sum += sright.parse::<i32>().unwrap();
        }
    }

    if spec_char.contains(same.as_str()) {
        if !spec_char.contains(sleft.as_str()) && sleft == sright {
            part_sum += sleft.parse::<i32>().unwrap();
        }
        else if !spec_char.contains(sleft.as_str()) && !spec_char.contains(sright.as_str()) {
            part_sum += sleft.parse::<i32>().unwrap();
            part_sum += sright.parse::<i32>().unwrap();
        }
    }

    if spec_char.contains(sright.as_str()) {
        if !spec_char.contains(same.as_str()) && same == sleft {
            part_sum += same.parse::<i32>().unwrap();
        }
        else if !spec_char.contains(same.as_str()) && !spec_char.contains(sleft.as_str()) {
            part_sum += same.parse::<i32>().unwrap();
            part_sum += sleft.parse::<i32>().unwrap();
        }
    }

    return part_sum;
}

fn part_number(schem: &Vec<Vec<char>>, spec_char: &HashSet<&str>, r: i32, c: i32, mut part_sum: i32) -> i32 {
    let row = r as usize;
    let col = c as usize;

    if (r-1) < 0 || r+1 < schem[row].len() as i32 {
        return part_sum;
    }

    // left 
    if !spec_char.contains(&schem[row][col-1].to_string().as_str()) {
        let mut str_num = String::new();
        let mut left = col-1;

        while !spec_char.contains(&schem[row][left].to_string().as_str()) && left > 0 {
            str_num = String::from(schem[row][left].to_string().as_str()) + &str_num;
            left -= 1;
        }

        part_sum += str_num.parse::<i32>().unwrap();
    }
    
    // right
    if !spec_char.contains(&schem[row][col+1].to_string().as_str()) {
        let mut str_num = String::new();
        let mut right = col+1;

        while !spec_char.contains(&schem[row][right].to_string().as_str()) && right < schem[row].len() {
            str_num.push_str( &String::from(schem[row][right].to_string().as_str()) );
            right += 1;
        }

        part_sum += str_num.parse::<i32>().unwrap();
    }

    // up and down
    part_sum = check(&schem, &spec_char, r-1, c, part_sum, "".to_string(), "".to_string(), "".to_string());
    part_sum = check(&schem, &spec_char, r+1, c, part_sum, "".to_string(), "".to_string(), "".to_string());

    part_sum
}

fn main() {
    let mut schematic: Vec<Vec<char>> = vec![];
    let special_char = HashSet::from(["*", "/", "!", "@", "#", "$", "%", "^", "=", "-", "&", "."]);
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            if let Ok(input) = line {
                let sch: Vec<_> = input.chars().collect();
                schematic.push(sch);
            }
        }
    }
    //println!("{:?}", schematic);
    let mut total = 0;
    
    for row in 0..schematic.len() {
        for col in 0..schematic[row].len() {
            //println!("{}", schematic[row][col]);
            if special_char.contains(&schematic[row][col].to_string().as_str()) {
                let r = row as i32;
                let c = col as i32;
                total += part_number(&schematic, &special_char, r, c, 0);
            }
        }
    }

    println!("{total}");

    //dfs(&schematic, &special_char);
}
