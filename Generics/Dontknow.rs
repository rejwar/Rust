use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let s = s.as_bytes();
        let n = s.len();
        let word_set = word_dict.iter().map(|s| s.as_bytes()).collect::<HashSet<_>>();
        let lens = word_set.iter().map(|s| s.len()).collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();

        let mut dp = vec![false; n + 1];
        dp[0] = true;
        for i in 1..=n {
            for len in lens.iter() {
                if i >= *len && dp[i-*len] && word_set.contains(&s[i-*len..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[n]
    }
}
