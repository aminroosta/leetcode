// I  1
// V  5
// X  10
// L  50
// C  100
// D  500
// M  1000
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num as usize;
        // we will map I to M to integer indices.
        let mapper = vec!['I', 'V', 'X', 'L', 'C', 'D', 'M'];
        // we will map 0 to 9 into above character indices
        let mut zerto_nine = vec![
            vec![],           //  0
            vec![0],          //  1 => I
            vec![0, 0],       //  2 => II
            vec![0, 0, 0],    //  3 => III
            vec![0, 1],       //  4 => IV
            vec![1],          //  5 => V
            vec![1, 0],       //  6 => VI
            vec![1, 0, 0],    //  7 => VII
            vec![1, 0, 0, 0], //  8 => VIII
            vec![0, 2],       //  9 => IX
        ];

        let mut res : Vec<usize> = vec![];
        while num != 0 {
            let idx = num % 10;
            res.extend(zerto_nine[idx].iter().rev());
            num /= 10;
            for vec in zerto_nine.iter_mut() {
                for e in vec.iter_mut() {
                    *e = *e + 2;
                }
            }
        }

        res.iter().rev()
           .map(|idx| mapper[*idx])
           .collect::<String>()
    }
}
