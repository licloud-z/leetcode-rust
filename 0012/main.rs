impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut answer = String::new();
        let mut num = num;
        while num / 1000 != 0 {
            answer.push('M');
            num = num - 1000;
        }
        if num / 900 != 0 {
            answer.push_str("CM");
            num = num - 900;
        }else {
            if num / 500 != 0{
                answer.push('D');
                num = num - 500;
            }else if num / 400 != 0 {
                answer.push_str("CD");
                num = num - 400;
            }
            while num / 100 != 0{
                answer.push('C');
                num = num - 100;
            }
        }

        if num / 90 != 0 {
            answer.push_str("XC");
            num = num - 90;
        }else {
            if num / 50 != 0{
                answer.push('L');
                num = num - 50;
            }else if num / 40 != 0 {
                answer.push_str("XL");
                num = num - 40;
            }
            while num / 10 != 0{
                answer.push('X');
                num = num - 10;
            }
        }

        if num / 9 != 0 {
            answer.push_str("IX");
            num = num - 9;
        }else {
            if num / 5 != 0{
                answer.push('V');
                num = num - 5;
            }else if num / 4 != 0 {
                answer.push_str("IV");
                num = num - 4;
            }
            while num / 1 != 0{
                answer.push('I');
                num = num - 1;
            }
        }
        answer
    }
}