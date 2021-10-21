impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut answer = 1;
        let mut right = nums[0];
        let mut left = 0;
        if nums.len() == 1 {return 0;}
        while right < nums.len() as i32 -1 {
            let current_right = right;
            for i in left..(right + 1) {
            //println!("{}",right);
                if i + nums[i as usize] > right{
                //left = i + 1;
                    right = i + nums[i as usize];
                
                }
                if right >= nums.len() as i32 - 1 {
                    break;
                }
            }
        //println!("{}",right);
            left = current_right + 1;
            answer = answer + 1;
        }
        answer
    }
}