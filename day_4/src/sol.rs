impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let chars = num.chars();
        let mut windows = num.windows(3);
        
        let mut max_num = 0;
        for win in windows {
            if win[0] == win[1] && win[1] == win[2] {
                let num: String = win.iter().collect();
                let pnum = num.parse().expect("Invalid num {num}");
                if pnum > max_num {
                    max_num = pnum;
                }
            }
        }
        return max_num;
    }
}