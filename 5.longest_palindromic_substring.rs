impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let a = s.as_bytes();
        let n = a.len();
        let mut dp = vec![vec![false; n]; n];
        let mut best_len = 0;
        let mut best_start = 0;

        for i in (0..a.len()).rev() {
            for j in (i..a.len()) {
                if a[i] == a[j] {
                   dp[i][j] = j <= i + 1 || dp[i+1][j-1];
                }
                if dp[i][j] && best_len < j - i + 1{
                    best_len = j - i + 1;
                    best_start = i;
                }
            }
        }

        String::from_utf8_lossy(&a[best_start..(best_start+best_len)]).into_owned()
    }
}
