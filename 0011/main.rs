impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left=0;
        let mut right = height.len() -1;
        let mut answer = 0;
        while left != right {
            if height[left] > height[right] {
                let temp_answer = height[right]*(right-left) as i32;
                if temp_answer > answer { answer = temp_answer; }
                right = right - 1;
            } else {
                let temp_answer = height[left]*(right-left) as i32;
                if temp_answer > answer { answer = temp_answer; }
                left = left + 1;
            }
        }
        answer
    }
}