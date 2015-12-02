extern crate network;

use network::{Capacity, Cost, NodeId};
use network::compact_star::{compact_star_from_edge_vec};
use network::algorithms::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

type Edge = (NodeId, NodeId, Cost, Capacity);

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = match args.len() {
        3 => &args[2],
        4 => &args[2],
        _ => {
            help();
            return;
        }
    };
    let start_node: NodeId = args[1].parse().unwrap();

    let mut node_to_id: HashMap<String, NodeId> = HashMap::new();
    let mut edges: Vec<Edge> = Vec::new();

    edges_from_file(Path::new(file_name), &mut node_to_id, &mut edges);
    let num_nodes = node_to_id.len();
    let compact_star = compact_star_from_edge_vec(num_nodes, &mut edges);
    let (pred, cost) = heap_dijkstra(&compact_star, start_node);



    // show results
    let number_to_show = if args.len() > 3 {
        match args[3].parse() {
            Ok(n) => n,
            Err(_) => num_nodes
        }
    } else {
        num_nodes
    };
    let mut id_to_node:HashMap<NodeId, String> = HashMap::with_capacity(num_nodes);
    for (node_name, id) in node_to_id.iter() {
        id_to_node.insert(id.clone(), node_name.clone());
    }
    for i in (0..num_nodes).take(number_to_show) {
        let from = pred.get(i).unwrap();
        let to = i as NodeId;
        let from_name = get_node_name(&from, &id_to_node);
        let to_name   = get_node_name(&to, &id_to_node);
        let cost_i = match cost.get(i as usize) {
            Some(c) => *c,
            None    => 0.0
        };
        println!("{} -> {}: {:.2}", from_name, to_name, cost_i);
    }
}

fn get_node_name(i: &NodeId, id_to_node: &HashMap<NodeId, String>) -> String {
    match id_to_node.get(i) {
        Some(name) => name.to_string(),
        None       => "NONE".to_string()
    }
}

fn edges_from_file<P>(filename: P, node_to_id: &mut HashMap<String,NodeId>, edges: &mut Vec<Edge>) 
        where P: AsRef<Path> {
    let f = BufReader::new(File::open(filename).ok().expect("Opening the file went bad."));
    let mut next_node: NodeId = 0;

    for line in f.lines().skip(1) {
        let l = match line {
            Ok(l) => l,
            Err(_) => return
        };
        let (from, to, cost, cap) = parse_line(&l, node_to_id, &mut next_node);
        edges.push((from, to, cost, cap));
        edges.push((to, from, cost, cap));
    }
}

fn parse_line(line: &str, node_to_id: &mut HashMap<String, NodeId>, next_node: &mut NodeId) -> Edge {
    let mut s = line.split_whitespace();
    let mut from_to_string = s.next().unwrap().split('.');
    let from_s = from_to_string.next().unwrap().to_string();
    let to_s = from_to_string.next().unwrap().to_string();
    let from = if node_to_id.contains_key(&from_s) {
        node_to_id[&from_s]
    } else {
        node_to_id.insert(from_s, *next_node);
        *next_node += 1;
        *next_node - 1
    };
    let to = if node_to_id.contains_key(&to_s) {
        node_to_id[&to_s]
    } else {
        node_to_id.insert(to_s, *next_node);
        *next_node += 1;
        *next_node - 1
    };
    let cost: Cost = s.next().unwrap().parse().unwrap();
    (from, to, cost, 0.0)
}

fn help() {
    println!("usage: test_network STARTNODE FILE [ENTRIES], 
              where FILE is the path to a file containing an edge list
              With format node_from.node_to  distance ...,
              STARTNODE is the id(!) of the node from where to start,
              and ENTRIES is the number of entries to print out.");
}

#[test]
fn test_breadth_first_search() {
    let test_edges = vec![(0,1,25.0,30.0),
                          (0,2,35.0,50.0), 
                          (1,3,15.0,40.0),
                          (2,1,45.0,10.0),
                          (3,2,15.0,30.0),
                          (3,4,45.0,60.0),
                          (4,2,25.0,20.0),
                          (4,3,35.0,50.0)];
    let compact_star = compact_star_from_edge_vec(5, test_edges);

    println!("breadth first search: {:?}", breadth_first_search(&compact_star, 0));
    println!("depth first search: {:?}", depth_first_search(&compact_star, 0));

    let test_edges = vec![
        (0,1,6.0,0.0),
        (0,2,4.0,0.0),
        (1,2,2.0,0.0),
        (1,3,2.0,0.0),
        (2,3,1.0,0.0),
        (2,4,2.0,0.0),
        (3,5,7.0,0.0),
        (4,3,1.0,0.0),
        (4,5,3.0,0.0)];
    let compact_star = compact_star_from_edge_vec(6, test_edges);
    println!("breadth first search: {:?}", heap_dijkstra(&compact_star, 0));
}
