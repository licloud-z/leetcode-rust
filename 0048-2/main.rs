impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = (*matrix).len();
        for i in 0..len{
            for j in 0..i{
                let temp = (*matrix)[i][j];
                (*matrix)[i][j] = (*matrix)[j][i];
                (*matrix)[j][i] = temp;
            }
        }
        for term in (*matrix).iter_mut(){
            term.reverse();
        }
    }
}