impl Solution {
    pub fn min_array(numbers: Vec<i32>) -> i32 {
        let mut left:usize = 0;
        let mut right:usize = numbers.len() - 1;
        if left == right { return numbers[left] ; }
        let mut mid = ( left + right ) / 2;
        while left < right {
        //println!("{},{}",left,right);
            if numbers[right] > numbers[left] { return numbers[left]; }
            if numbers[right] > numbers[mid] {
                right = mid;
                mid = ( left + right ) / 2;
            }else if numbers[right] == numbers[mid] {
                right = right - 1;
                mid = ( left + right ) / 2;
            }
            else {
                left = mid + 1;
                mid = ( left + right ) / 2;
            }
        }
        numbers[left]
    }
}