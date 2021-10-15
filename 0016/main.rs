impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        let mut answer= i32::MAX;
        let mut real_answer = 0;
        nums.sort();
    //println!("{:?}",nums);
        for i in 0..(nums.len() - 2) {
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }
            let wanted = target - nums[i];
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
                    let temp_answer = (get - wanted).abs();
                    if temp_answer < answer {
                        answer = temp_answer;
                        real_answer = nums[i] + nums[second] + nums[third];
                    //println!("{:?}",vec![nums[i],nums[second],nums[third]]);
                    }
                    third = third - 1;
                }else if get < wanted {
                    let temp_answer = (get - wanted).abs();
                    if temp_answer < answer {
                        answer = temp_answer;
                        real_answer = nums[i] + nums[second] + nums[third];
                    //println!("{:?}",vec![nums[i],nums[second],nums[third]]);
                    }
                    second = second + 1;
                }else {
                    return nums[i] + nums[second] + nums[third];
                }
            }
        }
        real_answer
    }
}