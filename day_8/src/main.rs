use std::{fs, fmt::Display};

#[derive(Debug)]
struct Move {
    root: String,
    left: String,
    right: String
}

fn main() {
    let content = fs::read_to_string("data/input3.txt").expect("File not found");

    let lines: Vec<&str> = content.split("\n").collect();


    let moves = lines[0];
    let mut map: Vec<Move> = Vec::new();
    for line in lines.into_iter().skip(2) {
        let tmp: Vec<&str> = line.split('=').collect();
        let root = tmp[0].trim();
        
        let lr_path: Vec<&str> = tmp[1].split(',').collect();
        let left = &lr_path[0].trim()[1..];
        let right = &lr_path[1].trim()[..lr_path[1].len()-2];

        
        map.push(
            Move {
                root: String::from(root),
                left: String::from(left),
                right: String::from(right)
            }
        )
    }
    let mut cur_move_idx: usize = 0;
    let mut curr_count: u32 = 0;
    let start_nodes: Vec<&Move> = map.iter().filter(|x| x.root.ends_with('A')).collect();
    println!("Start nodes len={}", start_nodes.len());

    let mut paths_sizes: Vec<u32> = Vec::new();

    for start_node in start_nodes.into_iter() {
        curr_count = 0;
        cur_move_idx = 0;
        let mut cur_node = start_node;
        let moves_list: Vec<char> = moves.chars().collect();
        println!("Starting with {:?}", start_node);
        while !cur_node.root.ends_with('Z') {
            //println!("root={}", cur_node.root);
            let m = moves_list[cur_move_idx];
            if m == 'L' {
                //println!("Moving L");
                let next_move = map.iter().find(|x| x.root == cur_node.left).unwrap();
                cur_node = next_move;
            } else {
                //println!("Moving R");
                let next_move = map.iter().find(|x| x.root == cur_node.right).unwrap();
                cur_node = next_move;
            }
            curr_count += 1;
            //println!("curr_count={}", curr_count);
            cur_move_idx = (cur_move_idx + 1) % moves_list.len();
        }
        println!("Last {:?}", cur_node);
        paths_sizes.push(curr_count);
    }
    //lcm(paths_sizes[0], paths_sizes[1]);
    println!("path_sizes={:?}", paths_sizes);

    //path_sizes=[20093 32262 54619 69618 82919 100182]
}
