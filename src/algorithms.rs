use super::{Network, NodeId, NodeVec};
use super::collections::{Collection, Queue, Stack};

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
/// let compact_star = compact_star_from_edge_vec(5, edges);
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
