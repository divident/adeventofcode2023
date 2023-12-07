use std::{fs, collections::{HashMap, HashSet}};

fn main() {
    let content = fs::read_to_string("data/input1.txt").expect("File not found");
    let lines = content.split("\n");
    let mut map: HashSet<i32> = HashSet::new();
    let mut count: HashMap<i32, i32> = HashMap::new();

    let mut fans = 0;
    for (gc, line) in lines.into_iter().enumerate() {
        let content: Vec<&str> = line.split(":").collect();
        let nums: Vec<&str> = content[1].split("|").collect();

        let win_nums: Vec<i32> = nums[0].split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().expect("Invalid num {x}"))
            .collect();
        let card_nums: Vec<i32> = nums[1].split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().expect("Invalid num {x}"))
            .collect();

        
        //println!("{:?}\n{:?}", win_nums, card_nums);
        for wn in win_nums {
            map.insert(wn);
        }

        let mut ans = 0;
        for cn in card_nums {
            if map.contains(&cn) {
                //println!("cn={cn}");
                ans += 1;
            }
        }
        let cur_multp = *count.get(&(gc as i32)).unwrap_or(&0) + 1;
        fans += cur_multp;

        println!("Game {gc}, ans={ans}, cur_multp={cur_multp}");
        while ans > 0 {
            *count.entry(gc as i32 + ans).or_insert_with(|| 0) += 1 * cur_multp;
            ans -= 1;
        }

        println!("{:?}", count);

        map.clear();
    }

    println!("fans={fans}");
}
