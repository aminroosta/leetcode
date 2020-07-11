impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut bst = 0;
        let mut cur = 0;
        let mut last_pos = [0; 256];
        for (idx, c) in s.bytes().enumerate() {
            cur = std::cmp::max(cur, last_pos[c as usize]);
            last_pos[c as usize] = idx + 1;
            bst = std::cmp::max(bst, idx + 1 - cur);
        }
        bst as i32
    }
}
