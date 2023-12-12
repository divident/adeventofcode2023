use std::{fs, collections::HashMap};

fn count(p: usize, g: usize, input: &Vec<char>, groups: &Vec<usize>, mem: &mut HashMap<(usize, usize), u64>) -> u64 {
    if mem.contains_key(&(p, g)) {
        return *mem.get(&(p, g)).unwrap();
    }
    if g >= groups.len() {
        if p < input.len() && input.into_iter().skip(p as usize).any(|x| *x == '#') {
            return 0;
        } else {
            return 1;
        }
    }

    let mut res: u64 = 0;
    let gs = groups[g];
    if p >= input.len() {
        return 0;
    }

    if p + gs >= input.len() {
        return 0;
    }

    if input[p] == '?' {

        // try starting group
        if input[p + gs] != '#' && input[p..p + gs].iter().all(|x| *x != '.') {
            res =  count(p + gs + 1, g + 1, input, groups, mem) + count(p + 1, g, input, groups, mem);
        } else {
            res = count(p + 1, g, input, groups, mem);
        }
    } else if input[p] == '#' {
        // start group here
        if input[p + gs] != '#' && input[p..p + gs].iter().all(|x| *x != '.')   {
            res = count(p + gs + 1, g + 1, input, groups, mem);
        } else {
            res = 0;
        }
    } else if input[p] == '.' {
        res = count(p + 1, g, input, groups, mem);
    }
    mem.insert((p, g), res);
    return res;
}
fn main() {
    let mut mem: HashMap<(usize, usize), u64> = HashMap::new();

    let content = fs::read_to_string("data/input1.txt").expect("File not found");
    let lines: Vec<&str> = content.split("\n").collect();
    let mut ans: u64 = 0;
    for line in lines.into_iter() {
        let groups: Vec<&str> = line.split(" ").collect();

        let parsed_line: Vec<char> = groups[0].chars().collect();
        let groups: Vec<usize> = groups[1].split(',').into_iter().map(|x| x.parse::<usize>().expect("Invalid num")).collect();
        
        let mut new_group: Vec<usize> = Vec::new();
        let mut new_line: Vec<char> = Vec::new();
        for i in 0..5 {
            for c in parsed_line.iter() {
                new_line.push(*c);
            }
            for g in groups.iter() {
                new_group.push(*g);
            }
            new_line.push('?');

        }

        mem.clear();
        let res = count(0, 0, &new_line, &new_group, &mut mem);

        ans += res;
    }
    
    println!("ans={ans}");
}
