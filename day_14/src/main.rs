use std::fs;
#[derive(Debug)]
struct Point2D {
    x: usize,
    y: usize,
}
fn main() {

    let content = fs::read_to_string("data/input1.txt").expect("File not found");

    let mut grid: Vec<Vec<char>> = Vec::new();

    let mut mov_stones: Vec<Vec<Point2D>> = Vec::new();

    let lines: Vec<&str> = content.split("\n").collect();

    for line in lines.into_iter() {
        grid.push(line.chars().collect());
    }

    // for line in grid.iter() {
    //     println!("{:?}", line);
    // }

    let n = grid.len();
    let m = grid[0].len();

    let mut ans: usize = 0;

    for i in 0..m {

        let mut stones: Vec<Point2D> = Vec::new();
        let mut blocks: Vec<Point2D> = Vec::new();

        for j in 0..n {
            if grid[j][i]  == 'O' {
                stones.push(Point2D{x: j, y: i});
            }

            if grid[j][i] == '#' {
                blocks.push(Point2D{x: j, y: i});

            }
        }

        blocks.push(Point2D{x: n, y: i});
        
        // println!("stones={:?}", stones);
        // println!("blocks={:?}", blocks);

        let mut curr_col = 0;
        let mut curr_block = 0;
        let mut new_stones: Vec<Point2D> = Vec::new();
        for stone in stones.into_iter() {
            let mut blocked = false;
            while curr_block < blocks.len() && blocks[curr_block].x < stone.x {
                curr_block+=1;
                blocked = true;
            } 

            if blocks.len() == 0 {
                new_stones.push(Point2D{x: curr_col, y: stone.y});
            }
            else if !blocked && blocks[curr_block].x > stone.x {
                new_stones.push(Point2D{x: curr_col, y: stone.y});
            } else if blocks[curr_block-1].x < stone.x {
                curr_col = blocks[curr_block-1].x + 1;
                new_stones.push(Point2D{x: curr_col, y: stone.y});
            }

            curr_col += 1;
        }

        // println!("{:?}", new_stones);
        ans += new_stones.iter().map(|x| n - x.x).sum::<usize>();

        mov_stones.push(new_stones);

    }

    println!("ans={}", ans);
}
