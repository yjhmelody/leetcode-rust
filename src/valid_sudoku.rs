#![allow(dead_code)]

// notes: can use array to replace the hash set to improve performance
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashSet;
    use std::iter;
    let mut col: Vec<HashSet<_>> = iter::repeat(HashSet::new()).take(board.len()).collect();
    let mut palaces: Vec<HashSet<_>> = iter::repeat(HashSet::new()).take(3).collect();
    for i in 0..board.len() {
        let mut row = HashSet::new();
        if i % 3 == 0 {
            palaces.iter_mut().for_each(|palace| {
                palace.clear();
            });
        }
        for j in 0..board[0].len() {
            let val = board[i][j];
            if val == '.' {
                continue;
            }
            let palace_num = j / 3;
            if row.contains(&val) || col[j].contains(&val) || palaces[palace_num].contains(&val) {
                return false;
            } else {
                row.insert(val);
                col[j].insert(val);
                palaces[palace_num].insert(val);
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let board = vec![
            ["5", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ];
        let board: Vec<Vec<char>> = board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&s| s.to_string().chars().nth(0).unwrap())
                    .collect()
            })
            .collect();
        assert_eq!(is_valid_sudoku(board), true);

        let board = vec![
            ["8", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ];
        let board: Vec<Vec<char>> = board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&s| s.to_string().chars().nth(0).unwrap())
                    .collect()
            })
            .collect();
        assert_eq!(is_valid_sudoku(board), false);
    }
}
