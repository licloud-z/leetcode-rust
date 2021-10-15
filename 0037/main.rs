impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut row = vec![vec![false;9];9];
        let mut column = vec![vec![false;9];9];
        let mut block = vec![vec![vec![false;9];3];3];
        let mut list = Vec::new();
        let mut sign =false;
        for i in 0..9{
            for j in 0..9{
                if (*board)[i][j] == '.' {
                    list.push([i,j]);
                }else {
                    let number = (*board)[i][j] as usize - '1' as usize;
                    row[i][number] = true;
                    column[j][number] = true;
                    block[i / 3][j / 3][number] = true;
                }
            }
        }
    //println!("{:?}",list);
        backtrack(board, 0, &mut row, &mut column, &mut block, &mut list, &mut sign);
    }
}

pub fn backtrack(board: &mut Vec<Vec<char>>, index:i32, row:&mut Vec<Vec<bool>>, column:&mut Vec<Vec<bool>>, block:&mut Vec<Vec<Vec<bool>>>, list:&mut Vec<[usize;2]>, mut sign:&mut bool){
    if index == (*list).len() as i32{
        (*sign) = true;
        return;
    }else {
        let [x, y] = (*list)[index as usize];
        let mut i = 0;
        while (*sign) == false && i < 9 {
            if (*row)[x][i] == false && (*column)[y][i] == false && (*block)[x / 3][y / 3][i] == false{
                (*row)[x][i] = true;
                (*column)[y][i] = true;
                (*block)[x / 3][y / 3][i] = true;
                (*board)[x][y] = ('1' as u8 + i as u8) as char;
                backtrack(board, index+1, row, column, block, list, sign);
                (*row)[x][i] = false;
                (*column)[y][i] = false;
                (*block)[x / 3][y / 3][i] = false;
            }
            i = i+1;
        }
    }
}