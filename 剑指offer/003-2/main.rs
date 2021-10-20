impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
		let mut nums = nums;
        for i in 0..nums.len() {
            //println!("{}",i);
            if i == nums[i] as usize {
                continue;
            }
            if nums[i] == nums[nums[i] as usize] {
                return nums[i];
            }
            //println!("{},{}",nums[i],nums[nums[i] as usize]);
            let index = nums[i] as usize;
            let temp = nums[i];
            nums[i] = nums[nums[i] as usize];
            nums[index] = temp;
            //println!("{:?}",nums);
        }
        0
    }
}