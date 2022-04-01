/**
 * [1925] Count Square Sum Triples
 *
 * A square triple (a,b,c) is a triple where a, b, and c are integers and a^2 + b^2 = c^2.
 * Given an integer n, return the number of square triples such that 1 <= a, b, c <= n.
 *  
 * Example 1:
 *
 * Input: n = 5
 * Output: 2
 * Explanation: The square triples are (3,4,5) and (4,3,5).
 *
 * Example 2:
 *
 * Input: n = 10
 * Output: 4
 * Explanation: The square triples are (3,4,5), (4,3,5), (6,8,10), and (8,6,10).
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 250
 *
 */
#[allow(dead_code)]
pub struct Solution {}

// problem: https://leetcode.com/problems/count-square-sum-triples/
// discuss: https://leetcode.com/problems/count-square-sum-triples/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(dead_code)]
impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut ans = 0;
        for a in 1..=n {
            for c in a + 1..=n {
                let b = c * c - a * a;
                if b >= n * n {
                    break;
                }
                let sqrt_b = (b as f64).sqrt() as i32;
                if b == sqrt_b * sqrt_b {
                    ans = ans + 1;
                }
            }
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {

    #[test]
    fn test_1925() {}
}
