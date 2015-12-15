use std::collections::HashMap;

use super::{Capacity, Cost, DoubleVec, NodeId, NodeVec, Network};

/// CompactStar representation of a network.
/// See: Ahuja, Magnati, Orlin: "Network Flows" for details.
#[derive(Debug, PartialEq)]
pub struct CompactStar { 
    point:      NodeVec,
    rpoint:     NodeVec,
    tail:       NodeVec,
    head:       NodeVec,
    trace:      NodeVec,
    costs:      DoubleVec,
    capacities: DoubleVec,
    cost_sum:   Cost
}

impl CompactStar {
    pub fn new(nodes: usize, edges: usize) -> CompactStar {
        CompactStar {
            point:      Vec::with_capacity(nodes+1),
            rpoint:     Vec::with_capacity(nodes+1),
            tail:       Vec::with_capacity(edges),
            head:       Vec::with_capacity(edges),
            trace:      Vec::with_capacity(edges),
            costs:      Vec::with_capacity(edges),
            capacities: Vec::with_capacity(edges),
            cost_sum:   0.0
        }
    }

    fn get_head(&self, from: NodeId, to: NodeId) -> Option<NodeId> {
        let i = from as usize;
        let lower = match self.point.get(i).map(|p| *p) {
            Some(value) => value as usize,
            None => return None
        };

        let upper = match self.point.get(i+1).map(|p| *p) {
            Some(value) => value as usize,
            None => return None
        };
    
        for index in lower..upper {
            if self.head.get(index).map(|p| *p).unwrap() == to {
                return Some(index as NodeId);
            }
        }

        None
    }

    fn get(&self, from: NodeId, to: NodeId, vec: &DoubleVec) -> Option<f64> {
        self.get_head(from,to)
            .and_then(|index| vec.get(index as usize))
            .map(|p| *p)
    }

}

impl Network for CompactStar {
    fn adjacent(&self, from: NodeId) -> Vec<NodeId> {
        let i = from as usize;
        let mut adj = Vec::new();
        let lower = match self.point.get(i).map(|p| *p) {
            Some(value) => value as usize,
            None => return adj
        };
        let upper = match self.point.get(i+1).map(|p| *p) {
            Some(value) => value as usize,
            None => return adj
        };
    
        for index in lower..upper {
            adj.push(self.head.get(index).unwrap().to_owned());
        }
        adj
    }

    fn cost(&self, i: NodeId, j: NodeId) -> Option<Cost> {
        self.get(i,j,&self.costs)
    }

    fn capacity(&self, i: NodeId, j: NodeId) -> Option<Capacity> {
        self.get(i,j,&self.capacities)
    }

    fn num_nodes(&self) -> usize {
        let n = self.point.len();
        if n > 0 {
            n - 1
        } else {
            n
        }
    }
    
    fn num_arcs(&self) -> usize {
        self.tail.len()
    }

    fn invalid_id(&self) -> NodeId {
        (self.point.len() - 1) as NodeId
    }

    fn infinity(&self) -> Cost {
        self.cost_sum
    }
}

/// Creates a network in compact star representation from a number of nodes and a list of edges.
///
/// # Arguments
/// * `nodes` - The number of unique node ids in the network. They have to be consecutively
/// numbered. That means, there are no gaps allowed.
/// * `edges` - (from, to, cost (length), capacity) tuples. These will be sorted by from-node
/// before building the compact star.
pub fn compact_star_from_edge_vec(nodes: usize, edges: &mut Vec<(NodeId, NodeId, Cost, Capacity)>) -> CompactStar {
    edges.sort_by(|&(n0, _, _, _), &(o0, _, _, _)| n0.cmp(&o0));
    let mut compact_star = CompactStar::new(nodes, edges.len());
    let mut tail_index = 0;
    let mut point_index = 0;

    let mut in_nodes: HashMap<NodeId, NodeVec> = HashMap::with_capacity(nodes);
    
    compact_star.point.push(tail_index);
    let mut edge_iter = edges.iter();
    loop {
        let &(from, to, cost, cap) = match edge_iter.next() {
            Some(tup) => tup,
            None => break
        };
        compact_star.tail.push(from);
        compact_star.head.push(to);
        compact_star.costs.push(cost);
        compact_star.cost_sum += cost;
        compact_star.capacities.push(cap);
        
        while point_index < from  {
            compact_star.point.push(tail_index);
            point_index += 1;
        }

        let mut in_node_edge = in_nodes.entry(to).or_insert(Vec::new());
        in_node_edge.push(tail_index);

        tail_index += 1;
    }

    let mut head_index = 0;
    for index in 0..nodes {
        let i = index as NodeId;
        compact_star.rpoint.push(head_index);
        if in_nodes.contains_key(&i) {
            for id in in_nodes.get(&i).unwrap() {
                compact_star.trace.push(*id);
                head_index += 1;
            }
        }
    }

    while point_index < nodes as NodeId - 1 {
        compact_star.point.push(tail_index);
        point_index += 1;
    }

    compact_star.point.push(tail_index);
    compact_star.rpoint.push(head_index);
    compact_star
}

// ====================================== TESTS ================================================

#[test]
fn access() {
    let mut compact_star = CompactStar::new(6,8);
    compact_star.point.push(1);
    assert_eq!(1, compact_star.point[0]);
}

#[test]
fn setup_sample_network() {
    let mut compact_star = CompactStar::new(6,8);
    for v in vec![0,2,3,4,6,8] { compact_star.point.push(v); }
    for v in vec![0,0,2,5,7,8] { compact_star.rpoint.push(v); }
    for v in vec![0,0,1,2,3,3,4,4] { compact_star.tail.push(v); }
    for v in vec![1,2,3,1,2,4,2,3] { compact_star.head.push(v); }
    for v in vec![25.0,35.0,15.0,45.0,15.0,45.0,25.0,35.0] { compact_star.costs.push(v); }
    for v in vec![30.0,50.0,40.0,10.0,30.0,60.0,20.0,50.0] { compact_star.capacities.push(v); }
    for v in vec![0,3,1,4,6,2,7,5] { compact_star.trace.push(v); }

    assert_eq!(5, compact_star.get_head(3,4).unwrap_or(0));
    assert_eq!(35.0, compact_star.cost(4,3).unwrap_or(0.0));
    assert_eq!(40.0, compact_star.capacity(1,3).unwrap_or(0.0));
    assert_eq!(5, compact_star.num_nodes());
    assert_eq!(vec![1,2], compact_star.adjacent(0));
    assert_eq!(vec![3], compact_star.adjacent(1));
    assert_eq!(vec![1], compact_star.adjacent(2));
    assert_eq!(vec![2,4], compact_star.adjacent(3));
    assert_eq!(vec![2,3], compact_star.adjacent(4));

    assert_eq!(5, compact_star.invalid_id());
}

#[test]
fn test_compact_star_from_edge_vec() {
    let mut comp_star_1 = CompactStar::new(5,8);
    for v in vec![0,2,3,4,6,8] { comp_star_1.point.push(v); }
    for v in vec![0,0,2,5,7,8] { comp_star_1.rpoint.push(v); }
    for v in vec![0,0,1,2,3,3,4,4] { comp_star_1.tail.push(v); }
    for v in vec![1,2,3,1,2,4,2,3] { comp_star_1.head.push(v); }
    for v in vec![25.0,35.0,15.0,45.0,15.0,45.0,25.0,35.0] { comp_star_1.costs.push(v); }
    for v in vec![30.0,50.0,40.0,10.0,30.0,60.0,20.0,50.0] { comp_star_1.capacities.push(v); }
    for v in vec![0,3,1,4,6,2,7,5] { comp_star_1.trace.push(v); }
    comp_star_1.cost_sum = 240.0;

    let mut edges = vec![(0,1,25.0,30.0),
                     (0,2,35.0,50.0),
                     (1,3,15.0,40.0),
                     (2,1,45.0,10.0),
                     (3,2,15.0,30.0),
                     (3,4,45.0,60.0),
                     (4,2,25.0,20.0),
                     (4,3,35.0,50.0)];
    let comp_star_2 = compact_star_from_edge_vec(5, &mut edges);
    
    assert_eq!(comp_star_1, comp_star_2);
}

#[test]
fn test_compact_start_from_edge_vec2() {
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
    assert_eq!(6, compact_star.num_nodes());
    assert_eq!(vec![0,2,4,6,7,9,9], compact_star.point);
}
