use std::collections::HashMap;

use network::{ DoubleVec, Network, NodeId };
use network::algorithms::{ dijkstra, pagerank };
use usage::{ DEFAULT_BETA, DEFAULT_EPS, DEFAULT_START_ID, Args };

#[derive(Debug, RustcDecodable)]
pub enum Algorithm { dijkstra, pagerank }

pub fn run_algorithm<N: Network>(network: &N, args: &Args, node_to_id: &HashMap<String, NodeId>) {
    match args.arg_algorithm {
        Algorithm::dijkstra => run_dijkstra(network, args, node_to_id),
        Algorithm::pagerank => run_pagerank(network, args, node_to_id),
    }
}

fn run_dijkstra<N: Network>(network: &N, args: &Args, node_to_id: &HashMap<String, NodeId>) {
    let start_id = match args.flag_start_node.as_ref() {
        Some(name) => node_to_id[name],
        None       => DEFAULT_START_ID,
    };
    let use_heap = args.flag_use_heap;
    let (pred, cost) = dijkstra(network, start_id, use_heap);
    print_dijkstra_result(&pred, &cost, &node_to_id)
}

fn run_pagerank<N: Network>(network: &N, args: &Args, node_to_id: &HashMap<String, NodeId>) {
    let beta = args.flag_beta.unwrap_or(DEFAULT_BETA);
    let eps = args.flag_eps.unwrap_or(DEFAULT_EPS);
    let ranks = pagerank(network, beta, eps);
    let target_node = args.flag_target_node.as_ref();
    print_pagerank_results(&ranks, node_to_id, target_node);
}

fn get_node_name(i: &NodeId, id_to_node: &HashMap<NodeId, String>) -> String {
    id_to_node.get(i).unwrap_or(&"NONE".to_string()).to_string()
}

fn print_dijkstra_result(pred: &Vec<NodeId>, cost: &DoubleVec, node_to_id: &HashMap<String, NodeId>) {
    let id_to_node: HashMap<NodeId, String> = node_to_id.iter()
        .map(|(k,v)| (*v,k.clone()))
        .collect();
    for i in (0..pred.len()).take(100) {
        let to_id = i as NodeId;
        let from_node = get_node_name(pred.get(i).unwrap(), &id_to_node);
        let to_node = get_node_name(&to_id, &id_to_node);
        let cum_cost = cost.get(i).unwrap();
        println!("{} -> {} : {:4}", from_node, to_node, cum_cost);
    }
}

fn print_pagerank_results(ranks: &Vec<f64>, node_to_id: &HashMap<String, NodeId>, target_node: Option<&String>) {
    match target_node {
        None => println!("No target node given."),
        Some(name) => {
            let id = node_to_id[name] as usize;
            println!("Rank of node {}: {} ({:e})", name, ranks[id], ranks[id]);
        }
    }
}
