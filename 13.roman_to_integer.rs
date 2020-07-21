 // I  1
 // V  5
 // X  10
 // L  50
 // C  100
 // D  500
 // M  1000
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut prev = 0;

        s.chars().rev() 
         .map(|c| match c {
            'I' =>  1,
            'V' =>  5,
            'X' =>  10,
            'L' =>  50,
            'C' =>  100,
            'D' =>  500,
            'M' =>  1000,
            _ => panic!()
        }).fold(0, |acc, x| {
             let r = acc + if x < prev { -x } else { x };
             prev = x;
             r
         })
    }
}
