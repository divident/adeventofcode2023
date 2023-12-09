use std::fs;

fn main() {
    let data = fs::read_to_string("data/input2.txt").expect("File not found");

    let lines: Vec<&str> = data.split("\n").collect();
    let mut ans = 0;
    for line in lines.into_iter() {
        println!("Parsing line={}", line);
        let mut nums: Vec<i32> = line.split(" ").into_iter().map(|x| x.parse::<i32>().expect("Invalid num {x}")).collect();
        nums.reverse();
        ans += parse_hisotry(nums);
    }

    println!("ans={ans}");
}

fn parse_hisotry(nums: Vec<i32>) -> i32 {
    let mut seq: Vec<Vec<i32>> = Vec::new();
    seq.push(nums);
    let mut all_zeros = false;
    while !all_zeros {
        let row = seq.last().unwrap();
        let next_row: Vec<i32> = row.windows(2).map(|w| w[1] - w[0]).collect();
        println!("new_row={:?}", next_row);
        all_zeros = next_row.iter().all(|x| *x == 0);
        seq.push(next_row);
    }

    let mut idx = seq.len() - 1;
    while idx > 0 {
        let new_el = seq[idx][seq[idx].len() - 1] + seq[idx-1][seq[idx-1].len() - 1];
        //println!("new_el={new_el}");
        seq[idx-1].push(new_el);
        idx -= 1;
    }

    return *seq[0].last().unwrap();
}

