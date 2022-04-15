/**
 * [79] Word Search
 *
 * Given an m x n grid of characters board and a string word, return true if word exists in the grid.
 * The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/word2.jpg" style="width: 322px; height: 242px;" />
 * Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
 * Output: true
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/word-1.jpg" style="width: 322px; height: 242px;" />
 * Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
 * Output: true
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/15/word3.jpg" style="width: 322px; height: 242px;" />
 * Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	m == board.length
 * 	n = board[i].length
 * 	1 <= m, n <= 6
 * 	1 <= word.length <= 15
 * 	board and word consists of only lowercase and uppercase English letters.
 *
 *  
 * Follow up: Could you use search pruning to make your solution faster with a larger board?
 *
 */
#[allow(dead_code)]
pub struct Solution {}

// problem: https://leetcode.com/problems/word-search/
// discuss: https://leetcode.com/problems/word-search/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(dead_code)]
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let (rows, cols) = (board.len(), board[0].len());
        let ref mut borad_clone = board.clone();

        for i in 0..rows {
            for j in 0..cols {
                if Solution::dfs(borad_clone, &word, i, j, 0) {
                    return true;
                }
            }
        }

        false
    }

    fn dfs(board: &mut Vec<Vec<char>>, word: &String, i: usize, j: usize, k: usize) -> bool {
        if k == word.len() {
            return true;
        }

        if i >= board.len() || j >= board[0].len() || j == usize::MAX || i == usize::MAX {
            return false;
        }

        if board[i][j] != word.chars().nth(k).unwrap() {
            return false;
        }

        board[i][j] = '\0';

        let ret = Solution::dfs(board, word, i + 1, j, k + 1)
            || Solution::dfs(board, word, i - 1, j, k + 1)
            || Solution::dfs(board, word, i, j + 1, k + 1)
            || Solution::dfs(board, word, i, j - 1, k + 1);

        board[i][j] = word.chars().nth(k).unwrap();

        return ret;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_79() {}
}
