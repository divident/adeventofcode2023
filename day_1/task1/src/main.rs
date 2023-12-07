use std::fs;

fn get_cords(line: &str) -> u32 {
    let nums_str: Vec<_> = Vec::from([
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ]);
    
    let mut ln: u32 = 0;
    let mut rn: u32 = 0;

    
    let mut min_num = line.len();
    let mut max_num = 0;
    for (idx, num) in nums_str.iter().enumerate() {
        let s = line.find(num);
        let sr = line.rfind(num);
        match s {
           Some(val) => {
            if val <= min_num {
                //println!("min={min_num} num={num} idx={idx}");
                min_num = val;
                ln = (idx + 1) as u32;
            }

           }
           _ => {continue;}
        }

        match sr {
            Some(val) => {
            if val >= max_num {
                //println!("max={max_num} num={num} idx={idx}");
                max_num = val;
                rn = (idx + 1) as u32;
            }
            }
            _ => {continue;}
        }

    }
    for (idx, el) in line.chars().enumerate() {
        match el {
            '0'..='9' => {
                if idx <= min_num {
                    ln = el.to_digit(10).unwrap();
                    min_num = idx;
                }

                if idx >= max_num {
                    rn = el.to_digit(10).unwrap();
                    max_num = idx;
                }
            },
            _ => continue
        }
    }
    //println!("ln={ln} rn={rn}");
    return ln * 10 + rn;
}
fn main() {
    let content = fs::read_to_string("data/test2.txt").expect("File not found");
    let lines = content.split("\n");
    let mut cords_sum = 0;
    for line in lines.into_iter() {

        //println!("{line}");
        //println!("{}", get_cords(line));
        cords_sum += get_cords(line);
    }
    println!("{cords_sum}");
    //println!("{content}");
}
