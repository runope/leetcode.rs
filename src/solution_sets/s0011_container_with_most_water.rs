/**
 * [11] Container With Most Water
 *
 * You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the i^th line are (i, 0) and (i, height[i]).
 * Find two lines that together with the x-axis form a container, such that the container contains the most water.
 * Return the maximum amount of water a container can store.
 * Notice that you may not slant the container.
 *  
 * Example 1:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/17/question_11.jpg" style="width: 600px; height: 287px;" />
 * Input: height = [1,8,6,2,5,4,8,3,7]
 * Output: 49
 * Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.
 *
 * Example 2:
 *
 * Input: height = [1,1]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	n == height.length
 * 	2 <= n <= 10^5
 * 	0 <= height[i] <= 10^4
 *
 */
#[allow(dead_code)]
pub struct Solution {}

// problem: https://leetcode.com/problems/container-with-most-water/
// discuss: https://leetcode.com/problems/container-with-most-water/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(dead_code)]
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0 as usize, height.len());
        let mut res = 0;

        while left < right {
            let volumn = (right - left) as i32 * height[left].min(height[right]);
            if volumn > res {
                res = volumn;
            }
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_11() {}
}
