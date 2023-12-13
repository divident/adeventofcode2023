use std::{collections::hash_map::DefaultHasher, fs};
use std::hash::{Hash, Hasher};

fn check(grid: &Vec<Vec<char>>) -> usize{
    let n = grid.len();

   
    for i in 1..n {

        let mut left = &grid[..i];
        let mut right = &grid[i..];

        if left.len() > right.len() {
            left = &left[(left.len() - right.len())..];
        } else if left.len() < right.len() {
            right = &right[..left.len()];
        }

        let mut rright: Vec<Vec<char>> = Vec::new();
        for el in right.iter().rev() {
            rright.push(el.to_vec());
        }



        right = &rright[..];
        
        // println!("right={:?} left={:?}", right, left);

        if left != right {
            
            let mut jright: Vec<char> = Vec::new();
            for el in right.iter() {
                jright.extend(el);
            }

            let mut jleft: Vec<char> = Vec::new();
            for el in left.iter() {
                jleft.extend(el);
            }

            let mut count = 0;
            if jleft.len() == jright.len() {
                for (a, b) in jleft.iter().zip(jright.iter()) {
                    if a != b {
                        count += 1;
                    }
                }
            }
            if count == 1 {
                return i;
            }

        }

    }

    return 0;

}
fn main() {
    let content = fs::read_to_string("data/input1.txt").expect("File not found");
    let patterns: Vec<&str> = content.split("\n\n").collect();
    
    let mut ans: u32 = 0;
    for patt in patterns.into_iter() {
        let lines: Vec<&str> = patt.split("\n").collect();
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in lines.into_iter() {
            grid.push(line.chars().collect());
        }

        let mut hash_horizontal: Vec<u64> = Vec::new();
        for line in grid.iter() {
            let ver_line: String  =  line.iter().collect();

            let mut hasher = DefaultHasher::new();
            ver_line.hash(&mut hasher);
            let hash_val = hasher.finish();
            hash_horizontal.push(hash_val);
        }

        let n = grid.len();
        let m = grid[0].len();

        let mut rgrid: Vec<Vec<char>> = Vec::new();

        // println!("grid");
        // for line in grid.iter() {
        //     println!("{:?}", line);
        // }

        for i in 0..m {
            let mut line: Vec<char> = Vec::new();
            for j in 0..n {
                line.push(grid[j][i]);
            }
            let s: Vec<char> = line.into_iter().collect();
            rgrid.push(s);
        }

        // println!("rgrid");
        // for line in rgrid.iter() {
        //     println!("{:?}", line);
        // }



        let rm = check(&grid);
        if  rm != 0 {
           ans += rm as u32 * 100; 
        } else {
            let m = check(&rgrid);
            ans += m as u32; 
        }
    }

    println!("ans={ans}");

}