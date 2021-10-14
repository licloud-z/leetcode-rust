impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut answer:Vec<Vec<i32>> = Vec::new();
        nums.sort();
    //println!("{:?}",nums);
        if nums.len() < 3 { return answer; }
        for i in 0..(nums.len() - 2) {
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }
            let wanted = 0 - nums[i];
            let mut second = i + 1;
            let mut third = nums.len() - 1;
            while second < third {
                if (second != (i+1)) && ( nums[second-1] == nums[second] ) {
                    second = second + 1;
                    continue;
                }
                let get = nums[second] + nums[third];
            //println!("{:?}",vec![nums[i],nums[second],nums[third]]);
                if get > wanted {
                    third = third - 1;
                }else if get < wanted {
                    second = second + 1;
                }else {
                    answer.push(vec![nums[i],nums[second],nums[third]]);
                    third = third - 1;
                    second = second + 1;
                }
            }
        }
        answer
    }
}