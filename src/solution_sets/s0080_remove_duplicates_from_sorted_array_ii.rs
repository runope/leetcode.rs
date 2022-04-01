/**
 * [80] Remove Duplicates from Sorted Array II
 *
 * Given an integer array nums sorted in non-decreasing order, remove some duplicates <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> such that each unique element appears at most twice. The relative order of the elements should be kept the same.
 * Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.
 * Return k after placing the final result in the first k slots of nums.
 * Do not allocate extra space for another array. You must do this by modifying the input array <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> with O(1) extra memory.
 * Custom Judge:
 * The judge will test your solution with the following code:
 * 
 * int[] nums = [...]; // Input array
 * int[] expectedNums = [...]; // The expected answer with correct length
 * int k = removeDuplicates(nums); // Calls your implementation
 * assert k == expectedNums.length;
 * for (int i = 0; i < k; i++) {
 *     assert nums[i] == expectedNums[i];
 * }
 * 
 * If all assertions pass, then your solution will be accepted.
 *  
 * Example 1:
 * 
 * Input: nums = [1,1,1,2,2,3]
 * Output: 5, nums = [1,1,2,2,3,_]
 * Explanation: Your function should return k = 5, with the first five elements of nums being 1, 1, 2, 2 and 3 respectively.
 * It does not matter what you leave beyond the returned k (hence they are underscores).
 * 
 * Example 2:
 * 
 * Input: nums = [0,0,1,1,1,1,2,3,3]
 * Output: 7, nums = [0,0,1,1,2,3,3,_,_]
 * Explanation: Your function should return k = 7, with the first seven elements of nums being 0, 0, 1, 1, 2, 3 and 3 respectively.
 * It does not matter what you leave beyond the returned k (hence they are underscores).
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 3 * 10^4
 * 	-10^4 <= nums[i] <= 10^4
 * 	nums is sorted in non-decreasing order.
 * 
 */
#[allow(dead_code)]
 pub struct Solution {}

 // problem: https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
 // discuss: https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/discuss/?currentPage=1&orderBy=most_votes&query=
 
 // submission codes start here
 
 #[allow(dead_code)]
 impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 && nums.len() == 1 && nums.len() == 2 {
            return nums.len() as i32;
        }
        
        let mut fast = 1;
        let mut index = 1;
        let mut tmp = (nums[0], false);

        while fast < nums.len() {
            while fast < nums.len() && nums[fast] == tmp.0 {
                tmp.1 = true;
                fast += 1;
            }

            if tmp.1 {
                nums[index] = tmp.0;
                index += 1;
                tmp.1 = false;
            } else {
                nums[index] = nums[fast];
                tmp.0 = nums[fast];
                index += 1;
                fast += 1;      
            }
        }

        index as i32
    }
}
 
 // submission codes end
 
 #[cfg(test)]
 mod tests {
     use super::*;
     use test_case::test_case;
 
     #[test_case(vec![1,1,1,2,2,3],&mut vec![1,1,2,2,3], 5; "example 1")]
     #[test_case(vec![0,0,1,1,1,1,2,3,3],&mut vec![0,0,1,1,2,3,3], 7; "example 2")]
     fn test_80(nums: Vec<i32>, target: &mut Vec<i32>, length: i32) {
        let ref mut nums = nums.clone();
        let ret_len = Solution::remove_duplicates(nums);

        let ref mut nums_vec = Vec::new();
        for i in 0..length {
            nums_vec.push(nums[i as usize]);
        }

         assert_eq!(ret_len, length);
         assert_eq!(nums_vec, target);
     }
 }
 