impl Solution {
    pub fn cutting_rope(n: i32) -> i32 {
        let mut answer = 1;
        if n == 1 { return 1; }
        else if n == 2 { return 1;}
        for i in 2..(n + 1) {
            let mut part1 = 0;
            let mut temp = 0;
            let mut newi = 0;
            if (n % i) <= (i / 2) {
                part1 = n / i;
                temp = n / i + n % i;
                newi = i;
            }else {
                part1 = n / i + 1;
                newi = n / part1;
                temp = part1 + n % part1;
                temp = (temp / 2) * (temp - (temp / 2));
            }
            //println!("{},{},  {}",part1,temp,i);
            for j in 1..newi {
                temp = temp * part1;
            }
            //println!("{}",temp);
            answer = std::cmp::max(answer, temp);
        } 
        answer
    }
}