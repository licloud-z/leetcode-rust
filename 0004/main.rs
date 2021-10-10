impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1,nums2) = if nums1.len() < nums2.len() {(nums1,nums2)} else {(nums2,nums1)};
        let len1 = nums1.len();
        let len2 = nums2.len();
        if (len1 + len2) == 0 {return 0.0;}
        let mut low = 0;
        let mut high = len1;
        let mut little = 0;
        let mut big = 0;
        while  low <= high {
            let i = (low + high) / 2;
            let j = (len1 + len2 + 1)/2 - i ;
        //println!("i:{},j:{}",i,j);
        //if j >= len2 {low = i+1;continue;} else if j < 0 {high = i-1;continue;}
            let num1_low = if i == 0 {i32::MIN} else {nums1[i-1]};
            let num1_high = if i == len1 {i32::MAX} else {nums1[i]};
            let num2_low = if j == 0 {i32::MIN} else {nums2[j-1]};
            let num2_high = if j == len2 {i32::MAX} else {nums2[j]};
        //println!("num1_low:{},num1_high:{}num2_low:{},num2_high:{}",num1_low,num1_high,num2_low,num2_high);
            if num2_high >= num1_low {
                little = if num1_low > num2_low {num1_low} else {num2_low};
                big = if num1_high > num2_high {num2_high} else {num1_high};
                low = i + 1;
            //println!("low:{},high:{}",low,high);
            }else {
                high = i - 1;
            }
        }
        let answer = if (len1 + len2) % 2 == 0 { (little + big) as f64/ 2.0 } else { little as f64};
        answer
    }
}