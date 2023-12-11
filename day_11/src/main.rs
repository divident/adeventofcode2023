use std::fs;
const INC: usize = 1000000 - 1;

fn abs(a: i32) -> i32 {
    if a < 0 {
        return -1 * a;
    } else {
        return a;
    }
}
#[derive(PartialEq, Debug)]
struct Point2D {
    x: usize,
    y: usize,
}

impl  Point2D {
    
    pub fn distance(&self, b: &Point2D) -> i32 {

        let dist = abs(self.x as i32 - b.x as i32) + abs(self.y as i32 - b.y as i32);
        return dist;
    }
}
fn expand_universe(grid: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut rows: Vec<usize> = vec![];

    for (i, row) in grid.iter().enumerate() {
        if row.iter().all(|x| *x == '.') {
            rows.push(i);
        }
    }

    println!("rows={:?}", rows);
    let n = grid.len();
    let m =grid[0].len();

    println!("n={n} m={m}");
    let mut cols: Vec<usize> = vec![];
    //let mut cur_row: usize = 0;
    for i in 0..m {
        let mut all_dots = true;
        for j in 0..n {
            if grid[j][i] == '#' {
                all_dots = false;
                break;
            }
        }
        if all_dots {
            cols.push(i);
        }
    }


    return (rows, cols);
}
fn main() {

    let content = fs::read_to_string("data/input3.txt").expect("File not found");
    let lines: Vec<&str> = content.split("\n").collect();

    let mut grid: Vec<Vec<char>> = vec![];

    for line in lines.into_iter() {
        let mut row: Vec<char> = vec![];

        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    // for row in grid.iter() {
    //     println!("{:?}", row);
    // }

    let (rows, cols) = expand_universe(&grid);


    // for row in exp_grid.iter() {
    //     println!("{:?}", row);
    // }

    let mut points: Vec<Point2D> = Vec::new();

    for (i, row) in grid.into_iter().enumerate() {
        for (j, c) in row.into_iter().enumerate() {
            if c == '#' {
                let idx = rows.partition_point(|&x| x < i);

                let idy = cols.partition_point(|&y| y < j);
                points.push(Point2D{x: i + INC * idx, y: j + INC * idy});

            }
        }
    }

    let mut ans: u64 = 0;
    let nn = points.len();
    let mut count_pairs = 0;
    for i in 0..nn {
        for j in i+1..nn {
            let a = &points[i];
            let b = &points[j];
            let dist =  a.distance(b) as u64;
            count_pairs += 1;
            //println!("Dist a={:?} b={:?} dist={}", a, b, dist);
            ans += dist;
        }
    }

    //println!("count_pairs={count_pairs}");

    //println!("points={:?}", points);

    println!("ans={}", ans);
}
