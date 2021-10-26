impl Solution {
    pub fn exchange(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 0 {return nums;}
        let mut right = nums.len() - 1;
        let mut left = 0;
        let mut nums = nums;
        while left < right {
            let lnum = nums[left];
            let lstate = lnum % 2 ;
            let rnum = nums[right];
            let rstate = rnum % 2;
            if lstate == 0 && rstate == 1 {
                let temp = nums[left];
                nums[left] = nums[right];
                nums[right] = temp;
                left += 1;
                right -= 1;
                //println!("{:?}",nums);
            }else if lstate == 0 && rstate == 0 {
                right -= 1;
            }else if rstate == 1 && rstate == 1{
                left += 1;
            }else {
                right -= 1;
                left += 1;
            }
        }
        nums
    }
}