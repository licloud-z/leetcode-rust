impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut ans = 0;
        merge_sort(0, nums.len(), &mut ans, &mut nums);
        ans
    }
}

pub fn merge_sort(left: usize, right: usize, ans: &mut i32, nums: &mut Vec<i32>){
    if right == left + 1 || right == left{
        return ;
    }else if right == left + 2 {
        if nums[right - 1] < nums[left]{
            *ans += 1;
            let temp = nums[left];
            nums[left] = nums[right - 1];
            nums[right - 1] = temp;
        }
    }else {
        let mid = (left + right) / 2;
        merge_sort(left, mid, ans, nums);
        //println!("{:?}",mid);
        merge_sort(mid, right, ans, nums);
        //println!("{:?}",nums);
        let mut first = left;
        let mut second = mid;
        //println!("{:?}",second);
        let mut tempvec = vec![0;right - left];
        tempvec.copy_from_slice(&nums[left..right]);
        //println!("{:?}",tempvec);
        for k in left..right {
            //println!("{}-{:?}",first,tempvec);
            if first == mid {
                nums[k] = tempvec[second-left];
                second += 1;
                //println!("{:?}-{}-{}",nums,first,second);
            }else if second == right || tempvec[first-left] <= tempvec[second-left]{
                nums[k] = tempvec[first-left];
                first += 1;
                //println!("{:?}-{}-{}",nums,first,second);
            }
            else{
                nums[k] = tempvec[second-left];
                second += 1;
                *ans += (mid - first ) as i32;
                //println!("{:?}-{}-{}",nums,first,second);
            }
        }
        //println!("{:?}",nums);
    }

}