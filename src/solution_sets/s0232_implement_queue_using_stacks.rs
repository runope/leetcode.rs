/**
 * [232] Implement Queue using Stacks
 *
 * Implement a first in first out (FIFO) queue using only two stacks. The implemented queue should support all the functions of a normal queue (push, peek, pop, and empty).
 * Implement the MyQueue class:
 * 
 * 	void push(int x) Pushes element x to the back of the queue.
 * 	int pop() Removes the element from the front of the queue and returns it.
 * 	int peek() Returns the element at the front of the queue.
 * 	boolean empty() Returns true if the queue is empty, false otherwise.
 * 
 * Notes:
 * 
 * 	You must use only standard operations of a stack, which means only push to top, peek/pop from top, size, and is empty operations are valid.
 * 	Depending on your language, the stack may not be supported natively. You may simulate a stack using a list or deque (double-ended queue) as long as you use only a stack's standard operations.
 * 
 *  
 * Example 1:
 * 
 * Input
 * ["MyQueue", "push", "push", "peek", "pop", "empty"]
 * [[], [1], [2], [], [], []]
 * Output
 * [null, null, null, 1, 1, false]
 * Explanation
 * MyQueue myQueue = new MyQueue();
 * myQueue.push(1); // queue is: [1]
 * myQueue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
 * myQueue.peek(); // return 1
 * myQueue.pop(); // return 1, queue is [2]
 * myQueue.empty(); // return false
 * 
 *  
 * Constraints:
 * 
 * 	1 <= x <= 9
 * 	At most 100 calls will be made to push, pop, peek, and empty.
 * 	All the calls to pop and peek are valid.
 * 
 *  
 * Follow-up: Can you implement the queue such that each operation is <a href="https://en.wikipedia.org/wiki/Amortized_analysis" target="_blank">amortized</a> O(1) time complexity? In other words, performing n operations will take overall O(n) time even if one of those operations may take longer.
 * 
 */
#[allow(dead_code)]
pub struct Solution {}

// problem: https://leetcode.com/problems/implement-queue-using-stacks/
// discuss: https://leetcode.com/problems/implement-queue-using-stacks/discuss/?currentPage=1&orderBy=most_votes&query=
 
// submission codes start here

#[allow(dead_code)]
struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    #[allow(unused)]
    fn new() -> Self {
        MyQueue { stack1: vec![], stack2: vec![] }
    }
    
    #[allow(unused)]
    fn push(&mut self, x: i32) {
        self.stack1.push(x);
    }
    
    #[allow(unused)]
    fn pop(&mut self) -> Option<i32> {
        if self.stack2.is_empty() {
            while !self.stack1.is_empty() {
                self.stack2.push(self.stack1.pop().unwrap());
            }
        }

        self.stack2.pop()
    }
    
    #[allow(unused)]
    fn peek(&self) -> Option<i32> {
        if let Some(x) = self.stack2.last() {
            return Some(x.clone());
        } else {
            return None;
        }
    }
    
    #[allow(unused)]
    fn empty(&self) -> bool {
        if self.stack1.is_empty() && self.stack2.is_empty() {
            true
        } else {
            false
        }
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

// submission codes end
 
#[cfg(test)]
mod tests {
#[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_232() {
        let mut my_queue = MyQueue::new();
        my_queue.push(1); // queue is: [1]
        my_queue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
        my_queue.peek(); // return 1
        my_queue.pop(); // return 1, queue is [2]
        my_queue.empty(); // return false
    }
}
 