/**
 * [167] Two Sum II - Input Array Is Sorted
 *
 * Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.
 * Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.
 * The tests are generated such that there is exactly one solution. You may not use the same element twice.
 * Your solution must use only constant extra space.
 *  
 * Example 1:
 * 
 * Input: numbers = [2,7,11,15], target = 9
 * Output: [1,2]
 * Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].
 * 
 * Example 2:
 * 
 * Input: numbers = [2,3,4], target = 6
 * Output: [1,3]
 * Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We return [1, 3].
 * 
 * Example 3:
 * 
 * Input: numbers = [-1,0], target = -1
 * Output: [1,2]
 * Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We return [1, 2].
 * 
 *  
 * Constraints:
 * 
 * 	2 <= numbers.length <= 3 * 10^4
 * 	-1000 <= numbers[i] <= 1000
 * 	numbers is sorted in non-decreasing order.
 * 	-1000 <= target <= 1000
 * 	The tests are generated such that there is exactly one solution.
 * 
 */
#[allow(dead_code)]
 pub struct Solution {}

 // problem: https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
 // discuss: https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/discuss/?currentPage=1&orderBy=most_votes&query=
 
 // submission codes start here
 
 #[allow(dead_code)]
 impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = (0, target - numbers[0]);
        let mut right = numbers.len() - 1;

        while left.0 < right {
            if numbers[right] > left.1 {
                right -= 1;
            } else if numbers[right] < left.1 {
                left.0 += 1;
                left.1 = target - numbers[left.0];
            } else {
                return vec![(left.0 + 1) as i32, (right + 1) as i32];
            }
        }

        vec![(left.0 + 1) as i32, (right + 1) as i32]
    }
}
 
 // submission codes end
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_167() {
         Solution::two_sum(vec![2,7,11,15], 9);
     }
 }
 