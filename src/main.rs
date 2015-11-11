extern crate network;

use network::compact_star::{compact_star_from_edge_vec};
use network::algorithms::*;

fn main() {
    let edges = vec![(0,1,25.0,30.0),
                     (0,2,35.0,50.0),
                     (1,3,15.0,40.0),
                     (2,1,45.0,10.0),
                     (3,2,15.0,30.0),
                     (3,4,45.0,60.0),
                     (4,2,25.0,20.0),
                     (4,3,35.0,50.0)];
    let compact_star = compact_star_from_edge_vec(5, edges);

    println!("breadth first search: {:?}", breadth_first_search(&compact_star, 0));
    println!("depth first search: {:?}", depth_first_search(&compact_star, 0));
}

#[test]
fn test_breadth_first_search() {
}
