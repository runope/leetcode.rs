/**
 * [209] Minimum Size Subarray Sum
 *
 * Given an array of positive integers nums and a positive integer target, return the minimal length of a contiguous subarray [numsl, numsl+1, ..., numsr-1, numsr] of which the sum is greater than or equal to target. If there is no such subarray, return 0 instead.
 *  
 * Example 1:
 * 
 * Input: target = 7, nums = [2,3,1,2,4,3]
 * Output: 2
 * Explanation: The subarray [4,3] has the minimal length under the problem constraint.
 * 
 * Example 2:
 * 
 * Input: target = 4, nums = [1,4,4]
 * Output: 1
 * 
 * Example 3:
 * 
 * Input: target = 11, nums = [1,1,1,1,1,1,1,1]
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	1 <= target <= 10^9
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^5
 * 
 *  
 * Follow up: If you have figured out the O(n) solution, try coding another solution of which the time complexity is O(n log(n)).
 */
#[allow(dead_code)]
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-size-subarray-sum/
// discuss: https://leetcode.com/problems/minimum-size-subarray-sum/discuss/?currentPage=1&orderBy=most_votes&query=
 
// submission codes start here

#[allow(dead_code)]
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut res = 100001;
        let (mut slow, mut fast) = (0, 0);
        while fast < nums.len() {
            sum += nums[fast];
            if sum < target {
                fast += 1;
            } else {
                res = res.min(fast - slow + 1);
                sum = sum - nums[slow] - nums[fast];
                slow += 1;
                if slow == fast + 1 {
                    break;
                }
            }
            
        }
        if res == 100001 {
            0
        } else {
            res as i32
        }
    }
}

// submission codes end
 
#[cfg(test)]
mod tests {
#[allow(unused_imports)]
    use super::*;
    use test_case::test_case;

    #[test_case(Solution::min_sub_array_len(7, vec![2,3,1,2,4,3]), 2)]
    #[test_case(Solution::min_sub_array_len(4, vec![1,4,4]), 1)]
    #[test_case(Solution::min_sub_array_len(11, vec![1,1,1,1,1,1,1,1]), 0)]
    fn test_209(res: i32, target: i32) {
        assert_eq!(res, target);
    }
}
 