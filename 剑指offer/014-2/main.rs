impl Solution {
    pub fn cutting_rope(n: i32) -> i32 {
        let mut answer = 1;
        let mut n = n;
        if n == 2 { return 1; }
        else if n == 3 { return 2;}
        while n - 3 > 1 {
            answer = answer * 3;
            n = n - 3;
        }
        answer * n
    }
}