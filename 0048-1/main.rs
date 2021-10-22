impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = (*matrix).len();
        let layer = len / 2;

        for lay in 0..(layer){
            for i in lay..( len - lay - 1) {  
                let temp = (*matrix)[len - 1 - lay][len - 1 - i];
                (*matrix)[len - 1 - lay][len - 1 - i] = (*matrix)[i][len - 1 - lay];
                (*matrix)[i][len - 1 - lay]           = (*matrix)[lay][i];
                (*matrix)[lay][i]                     = (*matrix)[len - 1 - i][lay];
                (*matrix)[len - 1 - i][lay]           = temp;
        //println!("{:?}",(*matrix));
            }
        }
    }
}