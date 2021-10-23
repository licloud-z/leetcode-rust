impl Solution {
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        let mut new_vec = vec![vec![0;n as usize+1];m as usize+1];
        new_vec[1][1] = 1;
        let mut answer = 1;
        for i in 1..(m + 1)as usize {
            for j in 1..(n + 1)as usize {
                let change_i = i-1;
                let change_j = j-1;
                let num_i = (change_i / 100 + change_i % 10 + (change_i / 10) % 10) as i32 ;
                let num_j = (change_j / 100 + change_j % 10 + (change_j / 10) % 10) as i32 ;
            //println!("{},{}",num_i,num_j);
                if (num_i + num_j <= k) && (new_vec[i-1][j] == 1 || new_vec[i][j-1] == 1) {
                    new_vec[i][j] = 1;
                    answer += 1;
                }
            //println!("{:?}",new_vec);
            }
        }
        answer
    }
}