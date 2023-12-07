
fn main() {

    let time: i64 = 56717999;
    let dist: i64 = 334113513502430;
    let mut ans = 1;
    ans = distance(time, dist);
    println!("{}", ans);
}

fn distance(t: i64, d: i64) -> u64 {
    let mut ans: u64 = 0;
    for x in 0..t {
        if -x*x + t * x - d > 0 {
            ans += 1;
        }
    }
    return ans;
}
