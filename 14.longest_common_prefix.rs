impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".into();
        }
        let mut i = 0;
        while i < strs[0].len() {
            let ch = strs[0].as_bytes()[i];
            for s in strs.iter() {
                if i >= s.as_bytes().len() || s.as_bytes()[i] != ch {
                    return strs[0][0..i].into();
                }
            }
            i = i+1;
        }

        strs[0][0..i].into()
    }
}
