use std::cmp::{Ord, Ordering};
use std::collections::BinaryHeap as RHeap;
use super::{ Cost, NodeId };

/// minimalistic heap trait restricted for `(NodeId, Cost)` tuples
/// 
/// It provides a more common interface than the original Rust implementation
/// suggests.
pub trait Heap {
    /// Find the min element in `O(1)` time.
    fn find_min(&self) -> Option<NodeId>;
    /// Return the current number of elements in the heap.
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn insert(&mut self, node_id: NodeId, cost: Cost);
    /// Remove the current minimal element.
    fn delete_min(&mut self);
}

/// BinaryHeap, wraps the native Rust implementation.
/// Rust's BinaryHeap is a max heap, we need a min heap.
/// I solved that by adding the costs as negative in the `insert` function.
pub struct BinaryHeap {
    inner_heap: RHeap<HeapMember>
}
impl BinaryHeap {
    pub fn new() -> Self {
        BinaryHeap {
            inner_heap: RHeap::new()
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        BinaryHeap {
            inner_heap: RHeap::with_capacity(capacity)
        }
    }
}

impl Heap for BinaryHeap {
    fn find_min(&self) -> Option<NodeId> {
        match self.inner_heap.peek() {
            Some(&member) => Some(member.key),
            None => None,
        }
    }
    fn size(&self) -> usize {
        self.inner_heap.len()
    }
    fn is_empty(&self) -> bool {
        self.inner_heap.is_empty()
    }
    /// Inserts a node with cost `-cost`. This turns the standard max heap
    /// as implemented in the Rust standard library into a min heap.
    fn insert(&mut self, node_id: NodeId, cost: Cost) {
        self.inner_heap.push(HeapMember { key: node_id, cost: -cost }) // rust heap is a max heap
    }
    fn delete_min(&mut self) {
        self.inner_heap.pop();
    }
}

/// Heap element, wraps a tuple of node id and respective costs
#[derive(Copy, Clone, Debug, PartialEq)]
struct HeapMember {
    key: NodeId,
    cost: Cost,
}

impl Eq for HeapMember {}

/// Implementation of `PartialOrd` based on the cost to reach a node
impl PartialOrd for HeapMember {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.cost.is_nan() || other.cost.is_nan() {
            return None;
        }
        if self.cost < other.cost {
            return Some(Ordering::Less);
        } else if self.cost > other.cost {
            return Some(Ordering::Greater);
        } else {
            return Some(Ordering::Equal);
        }
    }

    fn lt(&self, other: &Self) -> bool {
        self.cost < other.cost
    }
    fn le(&self, other: &Self) -> bool {
        self.cost <= other.cost
    }
    fn gt(&self, other: &Self) -> bool {
        self.cost > other.cost
    }
    fn ge(&self, other: &Self) -> bool {
        self.cost >= other.cost
    }
}

/// Implement a total ordering on elements of a heap based on costs
impl Ord for HeapMember {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost < other.cost {
            return Ordering::Less;
        } else if self.cost > other.cost {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }
}

#[test]
fn test_partial_ordering() {
    let mem1 = HeapMember{key: 0, cost: 0.0};
    let mem2 = HeapMember{key: 1, cost: 1.0};
    let mem3 = HeapMember{key: 2, cost: -1.0};

    assert!(mem1 < mem2);
    assert!(mem2 > mem1);
    assert!(mem3 < mem2);
    assert!(mem3 < mem1);
    assert!(mem1 == mem1);
}

#[test]
fn test_ordering() {
    let mem1 = HeapMember{key: 0, cost: 0.0};
    let mem2 = HeapMember{key: 1, cost: 1.0};
    let mem3 = HeapMember{key: 2, cost: -1.0};

    assert_eq!(Ordering::Less, mem1.cmp(&mem2));
    assert_eq!(Ordering::Greater, mem2.cmp(&mem1));
    assert_eq!(Ordering::Equal, mem1.cmp(&mem1));
    assert_eq!(Ordering::Less, mem3.cmp(&mem1));
}

#[test]
fn test_binary_heap() {
    let mut binary_heap = BinaryHeap::new();
    binary_heap.insert(0,0.0);
    assert_eq!(Some(0), binary_heap.find_min());
    binary_heap.insert(1,1.0);
    binary_heap.delete_min();
    binary_heap.insert(2,2.0);
    binary_heap.insert(3,3.0);
    assert_eq!(Some(1), binary_heap.find_min());
    assert_eq!(3, binary_heap.size());
    binary_heap.insert(4,4.0);
    binary_heap.insert(5,5.0);
    assert_eq!(5, binary_heap.size());
    assert_eq!(Some(1), binary_heap.find_min());
    binary_heap.insert(0,0.0);
    assert_eq!(Some(0), binary_heap.find_min());
}
