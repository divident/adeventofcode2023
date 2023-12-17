use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut an: HashMap<char, u32> = HashMap::new();
        let mut tc: HashMap<char, u32> = HashMap::new();

        for c in s.chars().into_iter() {
            *an.entry(c.to_owned()).or_default() += 1;
        }

        for c in t.chars().into_iter() {
            *tc.entry(c.to_owned()).or_default() += 1;
        } 

        let mut tcv: Vec<(char, u32)> = tc.into_iter().collect();

        let mut anv: Vec<(char, u32)> = an.into_iter().collect();

        tcv.sort();
        anv.sort();

        if tcv.len() != anv.len() {
            return false;
        }

        for (a, b) in tcv.into_iter().zip(anv.into_iter()) {
            if a.1 == b.1 && a.0 == b.0 {
                continue;
            } else {
                return false;
            }
        }


        return true;
    }
}
