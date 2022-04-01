/**
 * [172] Factorial Trailing Zeroes
 *
 * Given an integer n, return the number of trailing zeroes in n!.
 * Note that n! = n * (n - 1) * (n - 2) * ... * 3 * 2 * 1.
 *  
 * Example 1:
 *
 * Input: n = 3
 * Output: 0
 * Explanation: 3! = 6, no trailing zero.
 *
 * Example 2:
 *
 * Input: n = 5
 * Output: 1
 * Explanation: 5! = 120, one trailing zero.
 *
 * Example 3:
 *
 * Input: n = 0
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	0 <= n <= 10^4
 *
 *  
 * Follow up: Could you write a solution that works in logarithmic time complexity?
 *
 */
#[allow(dead_code)]
pub struct Solution {}

// problem: https://leetcode.com/problems/factorial-trailing-zeroes/
// discuss: https://leetcode.com/problems/factorial-trailing-zeroes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(dead_code)]
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut ret = 0;
        let mut n = n;

        if n < 5 {
            return 0;
        }

        while n >= 5 {
            ret += n / 5;
            n /= 5;
        }
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_172() {
        assert_eq!(5, Solution::trailing_zeroes(1));
        assert_eq!(3, Solution::trailing_zeroes(0));
        assert_eq!(0, Solution::trailing_zeroes(0));
        assert_eq!(10, Solution::trailing_zeroes(2));
        assert_eq!(30, Solution::trailing_zeroes(7));
    }
}
