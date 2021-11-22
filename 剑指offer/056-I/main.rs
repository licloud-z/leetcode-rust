impl Solution {
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut all = 0;
        for i in 0..nums.len(){
            all = all ^ nums[i];
        }
        let mut a = 1;
        while all & a == 0{
            a = a << 1;
        }
        let mut ans1 = 0;
        let mut ans2 = 0;
        for item in nums{
            if item & a == 0{
                ans1 = ans1 ^ item;
            }else{
                ans2 = ans2 ^ item;
            }
        }
        vec![ans1, ans2]
    }
}