use std::{fs, convert, cmp::max};
use regex::Regex;

fn get_values(line: &str) -> Option<i64> {

    let available_colors = vec![("blue", 14), ("red", 12), ("green", 13)];

    let lines = line.split(":").collect::<Vec<&str>>();
    let mut max_blue = 0;
    let mut max_red = 0;
    let mut max_green = 0;
    
    let tail = lines[1];
    let games = tail.split(";").collect::<Vec<&str>>();

    for game in games.into_iter() {
        let colors = game.split(",").collect::<Vec<&str>>();
        
        for col in colors.into_iter() {
            for av_col in available_colors.iter() {

                let idx = col.find((*av_col).0);
                let pcolor = (*av_col).0;
                println!("Color={pcolor}");
                match idx {
                    Some(val) => {
                        let col_count = &col[..val];

                        println!("col_count={col_count}");
                        match col_count.trim().parse::<i32>() {
                            Ok(val) => {
                                match pcolor {
                                    "green" => {
                                        max_green = max(max_green, val);
                                    },
                                    "blue" => {
                                        max_blue = max(max_blue, val);
                                    },
                                    "red" => {
                                        max_red = max(max_red, val);
                                    }
                                    _ => {println!("Color not found!");}
                                }
                            },
                            Err(e) => {
                                println!("One {e}");
                            }
                        }
                    }
                    None => {continue;}
                }
            }
        } 

    }
    println!("max_green={max_green} max_blue={max_blue} max_red={max_red}");
    return Some((max_green * max_blue * max_red) as i64);

}
fn main() {

    let content = fs::read_to_string("data/input2.txt").expect("File not found");
    
    let mut ans = 0;
    for line in content.split("\n").collect::<Vec<&str>>().into_iter() {

        println!("Parsing {line}");
        match get_values(line) {
            Some(val) => {
                println!("val={val}");
                ans += val;
            },
            _ => continue
        }

    }

    println!("ans={ans}");

}
