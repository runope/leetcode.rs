/**
 * [215] Kth Largest Element in an Array
 *
 * Given an integer array nums and an integer k, return the k^th largest element in the array.
 * Note that it is the k^th largest element in the sorted order, not the k^th distinct element.
 *  
 * Example 1:
 * Input: nums = [3,2,1,5,6,4], k = 2
 * Output: 5
 * Example 2:
 * Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
 * Output: 4
 *  
 * Constraints:
 * 
 * 	1 <= k <= nums.length <= 10^4
 * 	-10^4 <= nums[i] <= 10^4
 * 
 */
#[allow(dead_code)]
 pub struct Solution {}

 // problem: https://leetcode.com/problems/kth-largest-element-in-an-array/
 // discuss: https://leetcode.com/problems/kth-largest-element-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=
 
 // submission codes start here
 
 #[allow(dead_code)]
 impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut tmp_vec = vec![];
        for i in 0..k{
            tmp_vec.push(nums[i as usize]);
        };

        let mut min_index = Self::vec_min(&tmp_vec);

        for i in (k as usize)..nums.len() {
            if nums[i] > tmp_vec[min_index]{
                tmp_vec[min_index] = nums[i];
                min_index = Self::vec_min(&tmp_vec);
            }
        }

        tmp_vec[min_index] as i32 
    }

    pub fn vec_min(nums: &Vec<i32>) -> usize {
        let mut index = 0;
        for i in 0..nums.len() {
            if nums[i] < nums[index] {
                index = i;
            }
        }

        index
    }

    pub fn vec_max(nums: &Vec<i32>) -> usize {
        let mut index = 0;
        for i in 0..nums.len() {
            if nums[i] > nums[index] {
                index = i;
            }
        }

        index
    }
}
 
 // submission codes end
 
 #[cfg(test)]
 mod tests {
    #[allow(unused_imports)]
    use super::*;
 
     #[test]
     fn test_215() {
     }
 }
 