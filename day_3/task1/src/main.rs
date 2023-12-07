use std::{fs, collections::HashMap};

fn check_sign(x: usize, beg: usize, end: usize, grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut results: Vec<(usize, usize)> = Vec::new();

    let mut tbeg = if beg > 0 { beg - 1} else {beg};
    println!("beg={tbeg} end={end}");
    while tbeg <= end {
        //println!("beg={tbeg} end={end}");
        if (x as i32) - 1 >= 0 && tbeg < grid[x].len() {
            match grid[x - 1][tbeg] {
                '0'..='9' => {}
                '.' => {}
                _ => {
                    results.push((x-1, tbeg));
                }
            }
        }

        if x + 1 < grid.len() && tbeg < grid[x].len() {
            match grid[x + 1][tbeg] {
                '0'..='9' => {}
                '.' => {}
                _ => {
                    results.push((x+1, tbeg));
                }
            }
        }

        tbeg += 1;
    }


    if beg > 0 {
        match grid[x][beg - 1] {
            '0'..='9' => {}
            '.' => {}
            _ => {

                results.push((x, beg-1));
            }
        }
    }

    if end < grid[x].len() {
        match grid[x][end] {
            '0'..='9' => {}
            '.' => {}
            _ => {
                results.push((x, end));
            }
        }
    }
    return results;
}

fn main() {
    let mut map: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    let mut ans: i64 = 0;

    let content = fs::read_to_string("data/input2.txt").expect("File do not exits");
    let lines = content.split("\n");

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in lines.into_iter() {
        let chars = line.chars();
        let mut grid_line: Vec<char> = Vec::new();
        for char in chars.into_iter() {
            match char {
             '0'..='9' => {
                grid_line.push(char);
             },
             '*' => {
                grid_line.push(char);
             }
             '.' => {
                grid_line.push(char);
             }
             _ => {
                grid_line.push('.');
             }
            }
        }
        grid.push(grid_line);
    }

    let mut beg: i32 = -1;
    for i in 0..grid.len() {

        for j in 0..grid[0].len() {
            match grid[i][j] {
                '0'..='9' => {
                    if beg == -1 {
                        beg = j as i32;
                    }
                    if beg != -1 && j == grid[0].len() -1 {
                        let results = check_sign(i, beg as usize, j, &grid);
                        
                        if results.len() > 0 {
                            println!("outer beg={beg} end={j}");
                            let num = &grid[i][(beg as usize)..j+1];
                            let pnum: String = num.into_iter().collect();
                            let fnum = pnum.parse::<i32>().expect("Invalid number {pnum}");

                            for res in results.into_iter() {
                                if map.contains_key(&res) {
                                    let val = map.get_mut(&res).unwrap();
                                    val.push(fnum);
                                } else {
                                    map.insert(res, vec![fnum]);
                                }
                            }
                            println!("num={:?}", pnum);
                        }
                        beg = -1;
                    }
                }

                _ => {
                    if beg != -1 {
                        let results =  check_sign(i, beg as usize, j, &grid);
                        
                        if results.len() > 0 {
                            
                            println!("outer beg={beg} end={j}");
                            let num = &grid[i][(beg as usize)..j];
                            let pnum: String = num.into_iter().collect();
                            let fnum = pnum.parse::<i32>().expect("Invalid number {pnum}");

                            for res in results.into_iter() {
                                if map.contains_key(&res) {
                                    let val = map.get_mut(&res).unwrap();
                                    val.push(fnum);
                                } else {
                                    map.insert(res, vec![fnum]);
                                }
                
                            }

                            println!("num={:?}", pnum);
                        }
                        beg = -1;
                    }
                }
            }
        }

    }

    for (key, val) in &map {
        if val.len() == 2 {
            ans += (val[0] * val[1]) as i64;
        }
    }

    println!("{:?}", map);

    //517475 too high 512794
    println!("ans={ans}");
}
