use super::super::{ Network, NodeId };

/// Runs pagerank algorithm on a graph until convergence.
/// Convergence is reached, when the last ranks vector and the new one
/// differ by less than `eps` in their L1-norm.
/// `beta` is the teleport probability. CAUTION: Never use a teleport 
/// probability of `beta == 0.0`!!! Due to precision errors in the double
/// values, the sum of the ranks vector elements can exceed `1.0` which
/// will be caught by an assertion and the algorithm will panic.  
/// The result will be the pagerank for each node in the network.
pub fn pagerank<N: Network>(network: &N, beta: f64, eps: f64) -> Vec<f64> {
    let init_value = 1.0 / (network.num_nodes() as f64);
    let mut ranks = vec![0.0; network.num_nodes()];
    let mut new_ranks = vec![init_value; network.num_nodes()];
    let adj_lists = build_adj_list(network);
    let inv_out_deg = inv_out_deg(network);
    let mut i = 0;
    while !is_converged(&ranks, &new_ranks, eps) {
        print!("iteration {}: ", i);
        ranks = new_ranks;
        new_ranks = mult_matrix_vec(&adj_lists, &inv_out_deg, beta, &ranks);
        normalize(&mut new_ranks);
        i+=1;
    } 
    ranks
}
/// Calculates the inverse of the out degree for each node in the network.
/// For out degree `0`, the inverse will also be `0`, guaranteeing that we 
/// add `0.0` to the pagerank of the respective node.
fn inv_out_deg<N: Network>(network: &N) -> Vec<f64> {
    let mut inv_out_deg = Vec::with_capacity(network.num_nodes());
    for i in 0..network.num_nodes() {
        let out_deg = network.adjacent(i as NodeId).len() as f64;
        if out_deg > 0.0 {
            inv_out_deg.push(1.0 / out_deg);
        } else {
            inv_out_deg.push(0.0);
        }
    }
    inv_out_deg
}

/// Converts the network in a slightly faster traversable adjacency list.
fn build_adj_list<N: Network>(network: &N) -> Vec<Vec<usize>> {
    let mut adj_list = Vec::with_capacity(network.num_nodes());
    for i in 0..network.num_nodes() {
        let adj_nodes = network.adjacent(i as NodeId);
        let mut i_th_adj_nodes = Vec::with_capacity(adj_nodes.len());
        for j in adj_nodes {
            i_th_adj_nodes.push(j as usize);
        }
        adj_list.push(i_th_adj_nodes);
    }
    adj_list
}

/// Normalize the vector to \sum_i v_i = 1. Remaining mass is distributed 
/// evenly over all nodes. (Also known as smoothing.)
/// # Panics
/// If the sum of all elements is greater than `1.0`
fn normalize(vector: &mut Vec<f64>) {
    let mut sum = 0.0;
    for i in 0..vector.len() {
        sum += vector[i];
    }

    assert!(sum <= 1.0);
    let corrective_value = (1.0 - sum)/(vector.len() as f64);
    for i in 0..vector.len() {
        vector[i] += corrective_value;
    }
}

/// Multiply the ranks vector with the adjacency matrix. Every entry is 
/// damped by `1.0 - beta`. The vector is multiplied from the left!
fn mult_matrix_vec(adj_list: &Vec<Vec<usize>>, inv_out_degs: &Vec<f64>, beta: f64, current: &Vec<f64>) -> Vec<f64> {
    let mut new_ranks = vec![0.0; current.len()];
    for source_node in 0..current.len() {
        let inv_out_deg = inv_out_degs[source_node];
        for target_node in &adj_list[source_node] {
            new_ranks[*target_node] += (1.0-beta) * inv_out_deg * current[source_node];
        }
    }
    new_ranks
}

/// Determines convergence for two vectors with respect to the tolerance.
fn is_converged(old: &Vec<f64>, new: &Vec<f64>, eps: f64) -> bool {
    assert!(old.len() == new.len());
    let mut sum = 0.0;
    for i in 0..old.len() {
        sum += (old[i] - new[i]).powi(2);
    }
    println!("{:e} ({:e})", sum.sqrt(), eps);
    sum.sqrt() <= eps
}

#[test]
fn test_inv_out_deg() {
    use super::super::compact_star::compact_star_from_edge_vec;
    let mut edges = vec![
        (0,1,0.0,0.0),
        (0,2,0.0,0.0),
        (0,3,0.0,0.0),
        (1,2,0.0,0.0),
        (1,3,0.0,0.0),
        (2,0,0.0,0.0),
        (3,0,0.0,0.0),
        (3,2,0.0,0.0)];
    let compact_star = compact_star_from_edge_vec(4, &mut edges);
    assert_eq!(vec![1.0/3.0, 1.0/2.0, 1.0/1.0, 1.0/2.0], inv_out_deg(&compact_star));
}

#[test]
fn test_build_adj_list() {
    use super::super::compact_star::compact_star_from_edge_vec;
    let mut edges = vec![
        (0,1,0.0,0.0),
        (0,2,0.0,0.0),
        (0,3,0.0,0.0),
        (1,2,0.0,0.0),
        (1,3,0.0,0.0),
        (2,0,0.0,0.0),
        (3,0,0.0,0.0),
        (3,2,0.0,0.0)];
    let compact_star = compact_star_from_edge_vec(4, &mut edges);
    let adj_list = vec![vec![1,2,3], vec![2,3], vec![0], vec![0,2]];
    assert_eq!(adj_list, build_adj_list(&compact_star));
}

#[test]
fn test_normalize() {
    let mut to_normalize = vec![0.125, 0.125, 0.125, 0.125];
    normalize(&mut to_normalize);
    assert_eq!(vec![0.25, 0.25, 0.25, 0.25], to_normalize);
}

#[test]
fn test_is_converged() {
    let v1 = vec![0.0; 5];
    let v2 = vec![1.0; 5];
    let v3 = vec![1.0, 1.0, 1.0, 1.0, 1.00000001];
    assert!(is_converged(&v1, &v1, 1e-6));
    assert!(!is_converged(&v1, &v2, 1e-6));
    assert!(is_converged(&v2, &v3, 1e-4));
}

#[test]
fn test_pagerank() {
    use super::super::compact_star::compact_star_from_edge_vec;
    let mut edges = vec![
        (0,1,0.0,0.0),
        (0,2,0.0,0.0),
        (0,3,0.0,0.0),
        (1,2,0.0,0.0),
        (1,3,0.0,0.0),
        (2,0,0.0,0.0),
        (3,0,0.0,0.0),
        (3,2,0.0,0.0)];
    let compact_star = compact_star_from_edge_vec(4, &mut edges);
    let ranks = pagerank(&compact_star, 1e-10,1e-3);
    assert_eq!(vec![0.38,0.12,0.29,0.19], ranks);
}
