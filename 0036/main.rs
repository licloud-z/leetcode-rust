use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            let mut hashset: HashSet<char> = HashSet::new();
            for j in 0..9 {
                if board[i][j] == '.' {continue;}
                if hashset.contains(& board[i][j]) { return false; }
                hashset.insert(board[i][j]);
            }
        }

        for i in 0..9 {
            let mut hashset: HashSet<char> = HashSet::new();
            for j in 0..9 {
                if board[j][i] == '.' {continue;}
                if hashset.contains(& board[j][i]) { return false; }
                hashset.insert(board[j][i]);
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                let mut hashset: HashSet<char> = HashSet::new();
                for k in 0..3{
                    for l in 0..3{
                        if board[i * 3 + k][j * 3 + l] == '.' {continue;}
                        if hashset.contains(& board[i * 3 + k][j * 3 + l]) { return false; }
                        hashset.insert(board[i * 3 + k][j * 3 + l]);
                    }
                }
            }
        }

        true
    }
}