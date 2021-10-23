impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut word = word.as_bytes();
        let mut stage = vec![vec![0;board[0].len()];board.len()];
        let mut index = 0;
        if word.len() > board.len()*board[0].len() {return false;}
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if backtrack(& board, & word, 0, i, j, 4, &mut stage) == true {
                    return true;
                }
            }
        }
        false
    }
}

pub fn backtrack(board: & Vec<Vec<char>>, word: & [u8], index: usize, i: usize, j: usize, sign: i32, stage: &mut Vec<Vec<i32>>) -> bool{
    if index == (*word).len() {
        return true;
    }
    let mut space = vec![0;4];  //上、下、左、右
    if (*board)[i][j] == (*word)[index] as char {
        (*stage)[i][j] = 1;
        if sign < 4 { space[sign as usize] = 1; }
        if i == 0                       {space[0] = 1;}
        if i == ((*board).len() - 1)    {space[1] = 1;}
        if j == 0                       {space[2] = 1;}
        if j == (*board)[0].len() - 1   {space[3] = 1;}
        //println!("{:?}",space);
        if index + 1 == (*word).len() {
            return true;
        }
        if space[0] == 0 && (*stage)[i-1][j] == 0 && (*board)[i-1][j] == (*word)[index+1] as char{
            (*stage)[i-1][j] = 1;
            //println!("上");
            if backtrack(board, word, index+1, i-1, j, 1, stage) == true {return true;}
            (*stage)[i-1][j] = 0;
        }
        if space[1] == 0 && (*stage)[i+1][j] == 0 && (*board)[i+1][j] == (*word)[index+1] as char{
            (*stage)[i+1][j] = 1;
            //println!("下");
            if backtrack(board, word, index+1, i+1, j, 0, stage) == true {return true;}
            (*stage)[i+1][j] = 0;
        }
        if space[2] == 0 && (*stage)[i][j-1] == 0 && (*board)[i][j-1] == (*word)[index+1] as char{
            (*stage)[i][j-1] = 1;
            //println!("左");
            if backtrack(board, word, index+1, i, j-1, 3, stage) == true {return true;}
            (*stage)[i][j-1] = 0;
        }
        if space[3] == 0 && (*stage)[i][j+1] == 0 && (*board)[i][j+1] == (*word)[index+1] as char{
            (*stage)[i][j+1] = 1;
            //println!("右");
            if backtrack(board, word, index+1, i, j+1, 2, stage) == true {return true;}
            (*stage)[i][j+1] = 0;
        }
    }
    (*stage)[i][j] = 0;
    false
}