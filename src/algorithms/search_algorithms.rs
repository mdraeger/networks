use super::super::{Cost, DoubleVec, Network, NodeId, NodeVec};
use super::super::collections::{Collection, Queue, Stack};
use super::super::compact_star::compact_star_from_edge_vec;
use super::super::heaps::{ BinaryHeap, Heap };

/// Returns a tuple of node id lists as result of a Breadth-First search from node `start`. 
/// The first list is the predecessor list, that matches each node to it's predecessor in the
/// search path.
/// The second list is the order in which nodes are visited by the search algorithm.
/// # Arguments
/// * `network` a borrowed value that implements the Network trait.
/// * `start` a start node from where to search.
/// # Example
/// ```
/// let edges = vec![(0,1,25.0,30.0),
///                  (0,2,35.0,50.0),
///                  (1,3,15.0,40.0),
///                  (2,1,45.0,10.0),
///                  (3,2,15.0,30.0),
///                  (3,4,45.0,60.0),
///                  (4,2,25.0,20.0),
///                  (4,3,35.0,50.0)];
/// let compact_star = compact_star_from_edge_vec(5, &mut edges);
/// assert_eq!((vec![5,0,0,1,3],vec![0,1,2,3,4]));
/// ```
///
pub fn breadth_first_search<N: Network>(network: &N, start: NodeId) -> (NodeVec, NodeVec) {
    let n = network.num_nodes();
    let mut queue = Queue::with_capacity(n);
    search(network, &mut queue, start)
}

/// Returns a tuple of node id lists as result of a Depth-First search from node `start`. 
/// The first list is the predecessor list, that matches each node to it's predecessor in the
/// search path.
/// The second list is the order in which nodes are visited by the search algorithm.
/// # Arguments
/// * `network` a borrowed value that implements the Network trait.
/// * `start` a start node from where to search.
pub fn depth_first_search<N: Network>(network: &N, start: NodeId) -> (NodeVec, NodeVec) {
    let n = network.num_nodes();
    let mut stack = Stack::with_capacity(n);
    search(network, &mut stack, start)
}


fn search<C: Collection, N: Network>(network: &N, to_process: &mut C, start: NodeId) -> (NodeVec, NodeVec) {
    let n = network.num_nodes();
    let no_pred = network.invalid_id();
    let mut pred_slice = &mut (vec![no_pred; n])[..];
    let mut order_slice = &mut (vec![0; n])[..];
    let mut marks = &mut (vec![false; n])[..];

    let mut next: NodeId = 0;
    marks[start as usize] = true;
    order_slice[start as usize] = start;

    to_process.push(start);
    while !to_process.is_empty() {
        let i = *to_process.peek().unwrap();
        let adj = network.adjacent(i); let mut j = no_pred;
        for candidate in adj {
            if ! marks[candidate as usize] {
                j = candidate;
                break;
            }
        }
        if j != no_pred {
            marks[j as usize] = true;
            pred_slice[j as usize] = i;
            next += 1;
            order_slice[j as usize] = next;
            to_process.push(j);
        } else {
            to_process.pop();
        }
    }
    let mut pred = NodeVec::with_capacity(n);
    let mut order = NodeVec::with_capacity(n);
    for i in 0..n {
        pred.push(pred_slice[i]);
        order.push(order_slice[i]);
    }
    (pred, order)
}

pub fn dijkstra<N: Network>(network: &N, source: NodeId, use_heap: bool) -> (NodeVec, DoubleVec) {
  if use_heap {
    heap_dijkstra(network, source)
  } else {
    vanilla_dijkstra(network, source)
  }
}

pub fn vanilla_dijkstra<N: Network>(network: &N, source: NodeId) -> (NodeVec, DoubleVec) {
    let n = network.num_nodes();

    let mut temporary = NodeVec::with_capacity(n);
    for i in 0..n { temporary.push(i as NodeId); }

    let mut permanent = NodeVec::with_capacity(n);

    let pred = &mut (vec![network.invalid_id(); n])[..];
    let d = &mut (vec![network.infinity(); n])[..];
    d[source as usize] = 0.0;

    while permanent.len() < n {
        let next_node = find_min(&temporary, d);
        let index_in_temporary = find_min_index(&temporary, next_node);
        permanent.push(temporary.remove(index_in_temporary));
        for adjacent_node in network.adjacent(next_node) {
            let i = next_node as usize;
            let j = adjacent_node as usize;
            let cost = network.cost(next_node, adjacent_node).unwrap();
            if d[j] > d[i] + cost {
                d[j] = d[i] + cost;
                pred[j] = next_node;
            }
        }
    }

    // wrap it all up
    let mut pred_vec = NodeVec::with_capacity(n);
    let mut dist_vec = DoubleVec::with_capacity(n);
    for i in 0..n {
        pred_vec.push(pred[i]);
        dist_vec.push(d[i]);
    }
    (pred_vec, dist_vec)
}

fn find_min(to_check: &NodeVec, distances: &[Cost]) -> NodeId {
    let mut min = super::super::INF;
    let mut min_id = distances.len() as NodeId; // is invalid
    for node in to_check {
        let index = *node as usize;
        if distances[index] < min {
            min_id = *node;
            min = distances[index];
        }
    }
    min_id
}

fn find_min_index(list: &NodeVec, node: NodeId) -> usize {
    let mut index = 0;
    for i in 0..list.len() {
        if list.get(i) == Some(&node) {
           index = i;
        }
    }
    index
}

pub fn heap_dijkstra<N: Network> (network: &N, source: NodeId) -> (NodeVec, DoubleVec) {
    let n = network.num_nodes();

    let mut heap = BinaryHeap::new();
    let pred = &mut (vec![network.invalid_id(); n])[..];
    let d = &mut (vec![network.infinity(); n])[..];
    let marked = &mut(vec![false; n])[..];

    d[source as usize] = 0.0;
    heap.insert(source, 0.0);

    while !heap.is_empty() {
        let next_node = heap.find_min().unwrap();

        heap.delete_min(); // O(log n)
        let i = next_node as usize;

        if marked[i] {
            continue;
        }

        marked[i] = true;

        for adjacent_node in network.adjacent(next_node) {
            let cost = network.cost(next_node, adjacent_node).unwrap();
            let j = adjacent_node as usize;
            if d[j] > d[i] + cost {
                pred[j] = next_node;
                d[j] = d[i] + cost;
                heap.insert(adjacent_node, d[j]);
            }
        }
    }

    // wrap it all up
    let mut pred_vec = NodeVec::with_capacity(n);
    let mut dist_vec = DoubleVec::with_capacity(n);
    for i in 0..n {
        pred_vec.push(pred[i]);
        dist_vec.push(d[i]);
    }
    (pred_vec, dist_vec)
}

#[test]
fn test_dijkstra() {
    let mut edges = vec![
        (0,1,6.0,0.0),
        (0,2,4.0,0.0),
        (1,2,2.0,0.0),
        (1,3,2.0,0.0),
        (2,3,1.0,0.0),
        (2,4,2.0,0.0),
        (3,5,7.0,0.0),
        (4,3,1.0,0.0),
        (4,5,3.0,0.0)];
    let compact_star = compact_star_from_edge_vec(6, &mut edges);
    let (pred, dist) = dijkstra(&compact_star, 0, false);
    assert_eq!(6, pred.len());
    assert_eq!(6, dist.len());
    assert_eq!(vec![6,0,0,2,2,4], pred);
    assert_eq!(vec![0.0,6.0,4.0,5.0,6.0,9.0], dist);
}

#[test]
fn test_heap_dijkstra() {
    let mut edges = vec![
        (0,1,6.0,0.0),
        (0,2,4.0,0.0),
        (1,2,2.0,0.0),
        (1,3,2.0,0.0),
        (2,3,1.0,0.0),
        (2,4,2.0,0.0),
        (3,5,7.0,0.0),
        (4,3,1.0,0.0),
        (4,5,3.0,0.0)];
    let compact_star = compact_star_from_edge_vec(6, &mut edges);
    let (pred, dist) = dijkstra(&compact_star, 0, true);
    assert_eq!(6, pred.len());
    assert_eq!(6, dist.len());
    assert_eq!(vec![6,0,0,2,2,4], pred);
    assert_eq!(vec![0.0,6.0,4.0,5.0,6.0,9.0], dist);
}
