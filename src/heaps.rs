use std::cmp::{Ord, Ordering};
use std::collections::BinaryHeap as RHeap;
use super::{ Cost, NodeId, INF, NEG_INF };

/// minimalistic heap trait restricted for `(NodeId, Cost)` tuples
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
    fn new() -> Self {
        BinaryHeap {
            inner_heap: RHeap::new()
        }
    }
    fn with_capacity(capacity: usize) -> Self {
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
    fn insert(&mut self, node_id: NodeId, cost: Cost) {
        self.inner_heap.push(HeapMember { key: node_id, cost: -cost }) // rust heap is a max heap
    }
    fn delete_min(&mut self) {
        self.inner_heap.pop();
    }
}

struct FibElement {
    heap_member: HeapMember,
    rank: usize,
    children: Vec<FibElement>,
}

pub struct FibonacciHeap {
    min_elem: Option<FibElement>,
    root: Vec<FibElement>,
    size: usize,
}

impl FibonacciHeap {
    fn new() -> Self {
        FibonacciHeap {
            min_elem: None,
            root: Vec::new(),
            size: 0,
        }
    }
    fn remove_min(&mut self) {
        let min_elem = self.min_elem.take().unwrap();
        for child in min_elem.children {
            self.root.push(child);
        }
    }
    fn merge_roots(&mut self) {
        let root_degrees = &mut (vec![self.size; self.root.len()])[..]; // self.size is an invalid id
        while self.not_all_different() {
            for i in 0..self.root.len() {
                let rank = self.root.get(i).unwrap().rank;
                if root_degrees[rank] != self.size {
                    self.link(root_degrees[rank], i);
                    root_degrees[rank] = self.size;
                    break;
                } else {
                    root_degrees[rank] = i;
                }
            }
        }
    }
    fn update_min(&mut self) {
        let mut min_id = 0;
        let mut min_cost = INF;
        for i in 0..self.root.len() {
            let current_cost = self.root.get(i).unwrap().heap_member.cost;
            if current_cost < min_cost {
                min_cost = current_cost;
                min_id = i;
            }
        }
        self.min_elem = Some(self.root.remove(min_id))
    }
    fn link(&mut self, i: usize, j: usize) {
        let mut elem_i = self.root.remove(i);
        let mut elem_j = self.root.remove(j);
        if elem_i.heap_member < elem_j.heap_member {
            elem_i.children.push(elem_j);
            elem_i.rank += 1;
            self.root.push(elem_i);
        } else {
            elem_j.children.push(elem_i);
            elem_j.rank += 1;
            self.root.push(elem_j);
        }
    }
    fn not_all_different(&self) -> bool {
        let root_degrees = &mut (vec![self.size; self.root.len()])[..]; // self.size is an invalid id
        for i in 0..self.root.len() {
            let rank = self.root.get(i).unwrap().rank;
            if root_degrees[rank] != self.size {
                return true;
            } else {
                root_degrees[rank] = i;
            }
        }
        false
    } 
}

impl Heap for FibonacciHeap {
    fn find_min(&self) -> Option<NodeId> {
        self.min_elem.as_ref().map(|elem| elem.heap_member.key)
    }
    fn size(&self) -> usize {
        self.size
    }
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    fn insert(&mut self, node: NodeId, cost: Cost) {
        let new_elem = FibElement {
                heap_member: HeapMember{ key: node, cost: cost },
                rank: 0,
                children: Vec::new(),
            };

        if self.is_empty() {
            self.min_elem = Some(new_elem)
        } else if cost < self.min_elem.as_ref().unwrap().heap_member.cost {
            self.root.push(self.min_elem.take().unwrap());
            self.min_elem = Some(new_elem);
        } else {
            self.root.push(new_elem)
        }
        self.size += 1;
    }
    fn delete_min(&mut self) {
        if ! self.is_empty() {
            self.size -= 1;
            self.remove_min();
            self.merge_roots();
            self.update_min();
        }
    }
}

/// Heap element, wraps a tuple of node id and respective costs
#[derive(Copy, Clone, PartialEq)]
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

#[test]
fn test_fibonacci_heap() {
    let mut fibonacci_heap = FibonacciHeap::new();
    fibonacci_heap.insert(0,0.0);
    assert_eq!(Some(0), fibonacci_heap.find_min());
    fibonacci_heap.insert(1,1.0);
    fibonacci_heap.delete_min();
    fibonacci_heap.insert(2,2.0);
    fibonacci_heap.insert(3,3.0);
    assert_eq!(Some(1), fibonacci_heap.find_min());
    assert_eq!(3, fibonacci_heap.size());
    fibonacci_heap.insert(4,4.0);
    fibonacci_heap.insert(5,5.0);
    assert_eq!(5, fibonacci_heap.size());
    assert_eq!(Some(1), fibonacci_heap.find_min());
    fibonacci_heap.insert(0,0.0);
    assert_eq!(Some(0), fibonacci_heap.find_min());
}
