impl Solution {
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        /*let mut vec = Vec::new();
        for i in 0..n {
            vec.push(i);
        }
        let mut newm = (m as usize - 1) % vec.len();
        while vec.len() != 1 {
            vec.remove(newm);
            // 这个方法之所以不行，问题就在这里，remove()的时间复杂度并不是O(1),而是O(n)
            //println!("m:{}, vec:{:?}",newm,vec);
            newm = (m as usize - 1 + newm) % vec.len();
        }
        vec[0]*/
        let mut answer = 0;
        for i in 2..(n +1) {
            answer = (answer + m) % i;
        }
        answer
    }
}