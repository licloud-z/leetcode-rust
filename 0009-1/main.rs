impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {return false;}
        else if x == 0 {return true;}
        let mut x1 = x;
        let mut x2 = reverse(x1);
        if x1 == x2 {return true;}
        else {return false;}
    }
    
}

pub fn reverse(x: i32) -> i32 {
        let mut x1=x;
        let mut x2=0;
        let max_num = 2147483647;
        while x1 != 0 {
            if ((x2 == (max_num/10)) && ((x1%10) > 7)) || (x2 > (max_num/10)) {return 0;}
            x2 = x2*10 + x1%10;
            x1 = x1 / 10;
        }
        x2
    }