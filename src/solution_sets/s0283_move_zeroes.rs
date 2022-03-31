/**
 * [283] Move Zeroes
 *
 * Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
 * Note that you must do this in-place without making a copy of the array.
 *  
 * Example 1:
 * Input: nums = [0,1,0,3,12]
 * Output: [1,3,12,0,0]
 * Example 2:
 * Input: nums = [0]
 * Output: [0]
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^4
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 
 *  
 * Follow up: Could you minimize the total number of operations done?
 */
#[allow(dead_code)]
 pub struct Solution {}

 // problem: https://leetcode.com/problems/move-zeroes/
 // discuss: https://leetcode.com/problems/move-zeroes/discuss/?currentPage=1&orderBy=most_votes&query=
 
 // submission codes start here
 
 #[allow(dead_code)]
 impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[k] = nums[i];
                k += 1;
            }
        }

        for i in k..nums.len() {
            nums[i] = 0;
        }
    }
}
 
 // submission codes end
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_283() {
        let ref mut ex = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(ex);
        
     }
 }
 