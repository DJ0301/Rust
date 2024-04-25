impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut board = vec![vec!['.'; n as usize]; n as usize];
        Solution::place_queen(&mut result, &mut board, n as usize, 0);
        result
    }
    
    fn place_queen(result: &mut Vec<Vec<String>>, board: &mut Vec<Vec<char>>, n: usize, row: usize) {
        if row == n {
            let solution = board.iter().map(|row| row.iter().collect::<String>()).collect();
            result.push(solution);
            return;
        }
        
        for col in 0..n {
            if Solution::is_safe(board, row, col) {
                board[row][col] = 'Q';
                Solution::place_queen(result, board, n, row + 1);
                board[row][col] = '.';
            }
        }
    }
    
    fn is_safe(board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
        // Check the same column
        for i in 0..row {
            if board[i][col] == 'Q' {
                return false;
            }
        }
        
        // Check upper left diagonal
        let mut i = row as i32 - 1;
        let mut j = col as i32 - 1;
        while i >= 0 && j >= 0 {
            if board[i as usize][j as usize] == 'Q' {
                return false;
            }
            i -= 1;
            j -= 1;
        }
        
        // Check upper right diagonal
        let mut i = row as i32 - 1;
        let mut j = col as i32 + 1;
        while i >= 0 && j < board.len() as i32 {
            if board[i as usize][j as usize] == 'Q' {
                return false;
            }
            i -= 1;
            j += 1;
        }
        
        true
    }
}
