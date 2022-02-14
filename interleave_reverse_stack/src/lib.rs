/*
 Given a stack of N elements, interleave the first half of the stack
 with the second half reversed using only one other queue.
 This should be done in-place.

Recall that you can only push or pop from a stack, and enqueue or dequeue from a queue.
*/

use std::collections::VecDeque;

fn cycle_queue(queue: &mut VecDeque<i32>) {
    let mut i = 0;
    while i < queue.len() - 1 {
        let elem = queue.pop_front().unwrap();
        queue.push_back(elem);
        i+=1;
    }
}

pub fn interleave_reverse_stack(stack: &mut Vec<i32>) -> &Vec<i32> {
    if stack.len() < 3 {
        return stack;
    }
    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut i = 0;
    let stack_size = stack.len();
    while i < stack_size - 1 {
        queue.push_back(stack.pop().unwrap());
        i+=1;
    }
    stack.push(queue.pop_front().unwrap());

    while stack.len() < stack_size {
        cycle_queue(&mut queue);
        stack.push(queue.pop_front().unwrap());
        if stack.len() < stack_size {
            stack.push(queue.pop_front().unwrap());
        }
    }
    return stack
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interleave_reverse_test1() {
        let mut stack = vec![1, 2, 3, 4, 5];
        assert_eq!(*interleave_reverse_stack(&mut stack), [1, 5, 2, 4, 3]);
    }

    #[test]
    fn interleave_reverse_test2() {
        let mut stack = vec![1, 2, 3, 4];
        assert_eq!(*interleave_reverse_stack(&mut stack), [1, 4, 2, 3]);
    }

    #[test]
    fn interleave_reverse_test3() {
        let mut stack = vec![1, 2];
        assert_eq!(*interleave_reverse_stack(&mut stack), [1, 2]);
    }

    #[test]
    fn interleave_reverse_test4() {
        let mut stack = vec![1, 2, 3];
        assert_eq!(*interleave_reverse_stack(&mut stack), [1, 3, 2]);
    }
}
