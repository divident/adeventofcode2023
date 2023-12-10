// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

use std::{fs, collections::{VecDeque, HashSet}};


#[derive(Debug, PartialEq)]
enum Moves {
    NS, 
    EW,
    NE,
    NW,
    SW,
    SE,
    GR,
    ST
}

#[derive(Debug, PartialEq)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Point2D {
    x: i32,
    y: i32, 
}

fn abs(a: i32) -> i32 {
    if a < 0 {
        return -1 * a;
    } else {
        return a;
    }
}


impl Point2D {
    pub fn is_connected(self, move_: &Moves, other: Point2D, other_move: &Moves) -> bool {
        println!("move_={:?}", move_);
        return match move_ {
            Moves::ST =>  other.is_connected(other_move, self , move_),
            Moves::NS => other.x == self.x && abs(other.y - self.y) == 1,
            Moves::EW => other.y == self.y && abs(other.x - self.x) == 1,
            Moves::NE => (other.y == self.y - 1 && other.x == self.x) || (other.x == self.x + 1 && other.y == self.y),
            Moves::NW => (other.y == self.y - 1 && other.x == self.x) || (other.x == self.x - 1 && other.y == self.y),
            Moves::SW => (other.y == self.y + 1 && other.x == self.x) || (other.x == self.x - 1 && other.y == self.y),
            Moves::SE => (other.y == self.y + 1 && other.x == self.x) || (other.x == self.x + 1 && other.y == self.y),
            _              => false
        };
    } 
}
fn main() {
    let content = fs::read_to_string("data/input2.txt").expect("File not found");
    let lines: Vec<&str> = content.split("\n").collect();
    let mut grid: Vec<Vec<Moves>> = Vec::new();

    let mut start_pos: Point2D = Point2D { x: 0, y: 0 };
    for (i, line) in lines.iter().enumerate() {
        let mut cur_row: Vec<Moves> = vec![];

        for (j, char) in line.chars().into_iter().enumerate() {
            let cur_move = match char {
                '|' => Moves::NS,
                '-' => Moves::EW,
                'L' => Moves::NE,
                'J' => Moves::NW,
                '7' => Moves::SW,
                'F' => Moves::SE,
                'S' => Moves::ST,
                _ => Moves::GR,
            };
            if cur_move == Moves::ST {
                start_pos= Point2D {x: i as i32, y: j as i32};
            }
            cur_row.push(cur_move);
        }
        grid.push(cur_row);
    }
    let n = grid.len() as i32;
    let m = grid[0].len() as i32;

    let mut deq: VecDeque<Point2D> = VecDeque::from([start_pos, ]);
    let moves: Vec<Dir> = vec![Dir::UP, Dir::DOWN, Dir::LEFT, Dir::RIGHT];
    let mut visited: HashSet<Point2D> = HashSet::new();
    visited.insert(start_pos);
    let mut cur_len = 0;
    while let Some(top) = deq.pop_front() {

        println!("\ntop={:?}", top);
        for cur_dir in moves.iter() {
            let dd = match cur_dir {
                Dir::UP => (-1, 0),
                Dir::DOWN => (1, 0),
                Dir::LEFT => (0, -1),
                Dir::RIGHT => (0, 1),
            };

            let nx = top.x as i32 + dd.0;
            let ny = top.y as i32 + dd.1;
            let np = Point2D {x: nx, y: ny};
            println!("Move={:?} Checking={:?}", cur_dir, np);
            if 0 <= nx && nx < n  && 0 <= ny && ny < m {
                if top.is_connected(&grid[top.y as usize][top.x as usize], np, &grid[np.y as usize][np.x as usize]) && !visited.contains(&np) {
                    deq.push_back(np);
                    cur_len += 1;
                    visited.insert(np);
                }
            }
        }

    }

    println!("visited={:?}", visited);
    println!("ans={}", visited.len() / 2);
    println!("ans2={}", cur_len / 2);

    // for line in grid.iter() {
    //     println!("{:?}", line);
    // }

    let ans2 = part2(grid, &visited);
    println!("ans2={}", ans2);

}

fn part2(grid: Vec<Vec<Moves>>,  visited: &HashSet<Point2D>) -> i32 {
    let pipes = vec![Moves::NS, Moves::NE, Moves::NW, Moves::ST];
    let mut ans = 0;

   let n = grid.len();
   let m = grid[0].len();
   for y in 0..n {

    let mut inside = false;
    for x in 0..m {
        let np = Point2D{x: x as i32,y: y as i32};
        if visited.contains(&np) && pipes.iter().find(|a| **a == grid[y][x]).is_some() {
            inside = !inside;
        }

        if !visited.contains(&np) && inside {
            ans += 1;
        }
    }
   }
   return ans;
}