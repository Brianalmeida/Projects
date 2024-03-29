use std::iter::FromIterator;
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut res = vec![];
        Solution::dfs(n as usize, 0, 0, 0, &mut res, &mut vec![]);
        res
    }
    fn dfs(n: usize, diagonal_135: i32, diagonal_45: i32, col_mask: i32, res: &mut Vec<Vec<String>>, path: &mut Vec<usize>) {
        let bitmask = (1 << n) - 1;
        if bitmask == col_mask {
            res.push(Solution::decode(path, n));
            return;
        }
        let available = bitmask & (!(diagonal_135 | diagonal_45 | col_mask));
        for i in 0..n {
            let bit_info = 1 << i;
            if available & bit_info == 0 {
                continue;
            }
            path.push(i);
            Solution::dfs(
                n,
                (diagonal_135 | bit_info) >> 1,
                (diagonal_45 | bit_info) << 1,
                col_mask | bit_info,
                res,
                path, );
            path.pop();
        }
    }
    fn decode(path: &Vec<usize>, n: usize) -> Vec<String> {
        let board = path.iter().enumerate().fold(vec![vec!['.'; n]; n], |mut acc, (i, &j)| {
            acc[i][j] = 'Q';
            acc
        });
        board.iter().map(|x| String::from_iter(x)).collect()
    }
}
