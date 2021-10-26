impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut temp = x;
        let mut answer:f64 = 1.0;
        let mut n = n;
        if n > 0{
            while n != 0 {
                if (n & 1) == 1 {
                    answer = answer * temp;
                }
                temp = temp * temp;
                n = n >> 1;
            }
        }else{
            let mut sign = 0;
            if n == i32::MIN {
                sign = 1;
                n += 1;
            }
            n = n.abs();
            while n != 0 {
                if (n & 1) == 1 {
                    answer = answer * temp;
                }
                temp = temp * temp;
                n = n >> 1;
            }
            if sign == 1 { answer = answer * x; }
            answer = 1.0 / answer
        }
        answer
    }
}