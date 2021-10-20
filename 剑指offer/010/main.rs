impl Solution {
    pub fn num_ways(n: i32) -> i32 {
        if n <= 1 {return 1;}
        else if n == 2 {return 2;}
        let mut n1 = 1;
        let mut n2 = 2;
        let mut n3 = 0;
        let mut i = 2;
        while i < n {
            n3 = (n1 + n2) % 1000000007;
            n1 = n2 % 1000000007;
            n2 = n3 % 1000000007;
            i = i+1;
        }
        n3 % 1000000007
    }
}