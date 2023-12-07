use std::{
    cmp::{max, min},
    fs,
};
#[derive(Debug, Clone, Copy)]
struct Interval {
    beg_one: u64,
    beg_two: u64,
    end_one: u64,
    end_two: u64,
}

impl Interval {
    pub fn intersect(self, b: &Interval) -> bool {
        if self.end_two < b.beg_two || self.beg_two > b.end_two {
            return false;
        } else {
            return true;
        }
    }
}
fn main() {
    let content = fs::read_to_string("data/input1.txt").expect("File not found");

    let lines: Vec<&str> = content.split("\n").collect();

    let mut maps_beg: Vec<usize> = Vec::new();
    let mut intervals: Vec<Vec<Interval>> = Vec::new();
    let map_str = String::from("map:");
    for (i, line) in lines.iter().enumerate() {
        let idx = line.find(&map_str);
        match idx {
            Some(val) => {
                maps_beg.push(i);
            }
            _ => {}
        }
    }
    maps_beg.push(lines.len() + 1);
    println!("{:?}", maps_beg);
    let mut seeds: Vec<Interval> = Vec::new();
    let mut curr_map = 0;

    for i in 0..maps_beg.len() - 1 {
        let lines_slice = &lines[maps_beg[i] + 1..maps_beg[i + 1] - 1];
        let mut tmp_inter: Vec<Interval> = Vec::new();
        for ls in lines_slice.iter() {
            let nums: Vec<u64> = ls
                .split(" ")
                .map(|x| x.parse::<u64>().expect("Invalid num"))
                .collect();
            let interval = Interval {
                beg_one: nums[0],
                beg_two: nums[1],
                end_one: nums[0] + nums[2],
                end_two: nums[1] + nums[2],
            };
            tmp_inter.push(interval);
        }
        intervals.push(tmp_inter);
    }
    println!("{:?}", intervals);

    for (i, line) in lines.iter().enumerate() {
        let seed_str = String::from("seeds:");
        let mut idx = line.find(&seed_str);
        match idx {
            Some(val) => {
                let seed_nums: Vec<u64> = line[(val + seed_str.len() + 1)..]
                    .split(" ")
                    .map(|x| x.trim())
                    .map(|x| x.parse::<u64>().expect("Invalid num"))
                    .collect();
                for el in seed_nums.windows(2).into_iter() {
                    seeds.push(Interval {
                        beg_one: 0,
                        beg_two: el[0],
                        end_two: el[0] + el[1],
                        end_one: 0,
                    })
                }
            }
            _ => {}
        }
        if i == maps_beg[curr_map] {}
    }

    // get min of intervals including the cutoff
    let mut min_location: u64 = 999999999;

    let mut net_intv: Vec<Interval> = Vec::new();
    for seed in seeds.iter() {
        println!("++++++++++++++++++++++++++\nseed={:?}", seed);
        let mut cur_intv: Vec<Interval> = Vec::from([*seed]);

        for intv_list in intervals.iter() {
            for cur_val in cur_intv.iter() {
                println!("\ncur_val={:?}", cur_val);
                for intv in intv_list.iter() {
                    println!("intv={:?}\n", intv);
                    let n_beg = max(cur_val.beg_two, intv.beg_two);
                    let n_end = min(cur_val.end_two, intv.end_two);

                    println!("n_beg={n_beg} n_end={n_end}");
                    let diff_beg = n_beg - intv.beg_two;

                    let diff_end = intv.end_two - n_end;
                    println!("intv.beg_two={} end_two={}", intv.beg_two, intv.end_two);
                    println!("diff_beg={diff_beg} diff_end={diff_end}");

                    if n_beg != cur_val.beg_two {
                        net_intv.push(Interval {
                            beg_one: 0,
                            beg_two: cur_val.beg_two,
                            end_one: 0,
                            end_two: n_end - 1,
                        });
                    }

                    if n_end != cur_val.end_two {
                        net_intv.push(Interval {
                            beg_one: 0,
                            beg_two: n_end + 1,
                            end_one: 0,
                            end_two: cur_val.end_two,
                        });
                    }

                    net_intv.push(Interval {
                        beg_one: 0,
                        beg_two: intv.beg_one + diff_beg,
                        end_one: 0,
                        end_two: intv.end_one - diff_end,
                    });

                }
               
            }
            println!("net_intv={:?} cur_intv={:?}", net_intv, cur_intv);
            cur_intv = net_intv.clone();
            net_intv.clear();
        }
        println!("cur_intv={:?}", cur_intv);
        for cur_val in cur_intv.iter() {
            if min_location > cur_val.beg_two {
                min_location = cur_val.beg_two;
            }
        }
    }

    println!("min_location={min_location}")
}

fn abs(a: i64) -> i64 {
    if a < 0 {
        return -1 * a;
    } else {
        return a;
    }
}
