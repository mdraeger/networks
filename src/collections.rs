use std::collections::VecDeque;
use super::NodeId;

pub trait Collection {
    fn new() -> Self;
    fn with_capacity(init_cap: usize) -> Self;
    fn push(&mut self, element: NodeId);
    fn pop(&mut self) -> Option<NodeId>;
    fn peek(&self) -> Option<&NodeId>;
    fn is_empty(&self) -> bool;
}

pub struct Queue {
    data: VecDeque<NodeId>
}

pub struct Stack {
    data: VecDeque<NodeId>
}

impl Collection for Queue {
    fn new() -> Queue {
        Queue { 
            data: VecDeque::new() 
        }
    }

    fn with_capacity(init_cap: usize) -> Queue {
        Queue {
            data: VecDeque::with_capacity(init_cap)
        }
    }

    fn push(&mut self, element: NodeId) {
        self.data.push_back(element)
    }

    fn pop(&mut self) -> Option<NodeId> {
        self.data.pop_front()
    }

    fn peek(&self) -> Option<&NodeId> {
        self.data.front()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Collection for Stack {
    fn new() -> Stack {
        Stack { 
            data: VecDeque::new() 
        }
    }

    fn with_capacity(init_cap: usize) -> Stack {
        Stack {
            data: VecDeque::with_capacity(init_cap)
        }
    }

    fn push(&mut self, element: NodeId) {
        self.data.push_back(element)
    }

    fn pop(&mut self) -> Option<NodeId> {
        self.data.pop_back()
    }

    fn peek(&self) -> Option<&NodeId> {
        self.data.back()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[test]
fn test_queue_impl() {
    let mut queue = Queue::new();
    queue.push(0);
    queue.push(1);
    queue.push(2);
    queue.push(3);
    queue.push(4);
    assert!(! queue.is_empty());
    assert_eq!(Some(0), queue.pop());
    assert_eq!(Some(&1), queue.peek());
    queue.pop();
    queue.pop();
    queue.pop();
    queue.pop();
    assert!(queue.is_empty());
}

#[test]
fn test_stack_impl() {
    let mut stack = Stack::new();
    stack.push(0);
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    assert!(! stack.is_empty());
    assert_eq!(Some(4), stack.pop());
    assert_eq!(Some(&3), stack.peek());
    stack.pop();
    stack.pop();
    stack.pop();
    stack.pop();
    assert!(stack.is_empty());
}
