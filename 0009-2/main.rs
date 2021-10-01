impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {return false;}
        else if x < 10 {return true;}
        //如果x最低位是0，就会导致下面的x2无法进位
        else if x%10 == 0 {return false;}
        let mut x1 = x;
        let mut x2 = 0;
        while x1 > x2 {
            x2 = x2*10 + x1%10;
            x1 = x1 / 10;
        }
        if (x1 == x2) || ((x2/10) == x1) {return true;}
        else {return false;}
    }
    
}