/**
 * [75] Sort Colors
 *
 * Given an array nums with n objects colored red, white, or blue, sort them <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> so that objects of the same color are adjacent, with the colors in the order red, white, and blue.
 * We will use the integers 0, 1, and 2 to represent the color red, white, and blue, respectively.
 * You must solve this problem without using the library's sort function.
 *  
 * Example 1:
 *
 * Input: nums = [2,0,2,1,1,0]
 * Output: [0,0,1,1,2,2]
 *
 * Example 2:
 *
 * Input: nums = [2,0,1]
 * Output: [0,1,2]
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 300
 * 	nums[i] is either 0, 1, or 2.
 *
 *  
 * Follow up: Could you come up with a one-pass algorithm using only constant extra space?
 *
 */
#[allow(dead_code)]
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-colors/
// discuss: https://leetcode.com/problems/sort-colors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[allow(dead_code)]
impl Solution {
    // pub fn sort_colors(nums: &mut Vec<i32>) {
    //     let mut zero = 0;
    //     let mut one = 0;
    //     let mut two = 0;

    //     for i in nums.iter() {
    //         match i {
    //             0 => zero += 1,
    //             1 => one += 1,
    //             2 => two += 1,
    //             _ => (),
    //         }
    //     }

    //     for i in 0..nums.len() {
    //         if zero > 0 {
    //             nums[i] = 0;
    //             zero -= 1;
    //         } else if one > 0 {
    //             nums[i] = 1;
    //             one -= 1;
    //         } else {
    //             nums[i] = 2;
    //             two -= 1;
    //         }
    //     }
    // }

    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut lp = 0;
        let mut i = 0;
        let mut rp = nums.len();

        while i < rp {
            if nums[i] == 0 {
                nums.swap(i, lp);
                lp += 1;
                i += 1;
            } else if nums[i] == 2 {
                rp -= 1;
                nums.swap(i, lp);
            } else {
                i += 1;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_75() {}
}
