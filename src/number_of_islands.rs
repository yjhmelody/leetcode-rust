#![allow(dead_code)]

// dfs with recursive: time out
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    use std::iter;
    let mut visited: Vec<Vec<bool>> = Vec::with_capacity(grid.len());
    for _ in 0..grid.len() {
        visited.push(iter::repeat(false).take(grid[0].len()).collect());
    }
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' && !visited[i][j] {
                dfs(&grid, &mut visited, i, j);
                count += 1;
            }
        }
    }

    count
}

fn dfs(grid: &Vec<Vec<char>>, mut visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    visited[i][j] = true;

    if i < grid.len() - 1 && !visited[i + 1][j] && grid[i + 1][j] == '1' {
        dfs(&grid, &mut visited, i + 1, j);
    }
    if i > 0 && !visited[i - 1][j] && grid[i - 1][j] == '1' {
        dfs(&grid, &mut visited, i - 1, j);
    }
    if j < grid[0].len() - 1 && !visited[i][j + 1] && grid[i][j + 1] == '1' {
        dfs(&grid, &mut visited, i, j + 1);
    }
    if j > 0 && !visited[i][j - 1] && grid[i][j - 1] == '1' {
        dfs(&grid, &mut visited, i, j - 1);
    }
}

// bfs with loop: time out
pub fn num_islands2(grid: Vec<Vec<char>>) -> i32 {
    use std::iter;
    let mut visited: Vec<Vec<bool>> = Vec::with_capacity(grid.len());
    for _ in 0..grid.len() {
        visited.push(iter::repeat(false).take(grid[0].len()).collect());
    }
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' && !visited[i][j] {
                bfs(&grid, &mut visited, i, j);
                count += 1;
            }
        }
    }

    count
}

fn bfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    use std::collections::LinkedList;
    let mut q = LinkedList::new();
    q.push_back((i, j));
    while !q.is_empty() {
        match q.pop_front() {
            Some((i, j)) => {
                visited[i][j] = true;
                if i + 1 < grid.len() && !visited[i + 1][j] && grid[i + 1][j] == '1' {
                    q.push_back((i + 1, j));
                }
                if i > 0 && !visited[i - 1][j] && grid[i - 1][j] == '1' {
                    q.push_back((i - 1, j));
                }
                if j + 1 < grid[0].len() && !visited[i][j + 1] && grid[i][j + 1] == '1' {
                    q.push_back((i, j + 1));
                }
                if j > 0 && !visited[i][j - 1] && grid[i][j - 1] == '1' {
                    q.push_back((i, j - 1));
                }
            }

            None => unreachable!()
        }
    }
}


// bfs with loop: reuse the grid
pub fn num_islands3(mut grid: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' {
                bfs2(&mut grid, i, j);
                count += 1;
            }
        }
    }

    count
}

fn bfs2(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
    use std::collections::LinkedList;
    let mut q = LinkedList::new();
    q.push_back((i, j));
    while !q.is_empty() {
        match q.pop_front() {
            Some((i, j)) => {
                if i + 1 < grid.len() && grid[i + 1][j] == '1' {
                    grid[i + 1][j] = '0';
                    q.push_back((i + 1, j));
                }
                if i > 0 && grid[i - 1][j] == '1' {
                    grid[i - 1][j] = '0';
                    q.push_back((i - 1, j));
                }
                if j + 1 < grid[0].len() && grid[i][j + 1] == '1' {
                    grid[i][j + 1] = '0';
                    q.push_back((i, j + 1));
                }
                if j > 0 && grid[i][j - 1] == '1' {
                    grid[i][j - 1] = '0';
                    q.push_back((i, j - 1));
                }
            }

            None => unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        assert_eq!(num_islands(grid), 3);
    }

    #[test]
    fn test2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        assert_eq!(num_islands2(grid), 3);
    }

    #[test]
    fn test3() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        assert_eq!(num_islands3(grid), 3);
    }
}