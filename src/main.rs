//   Copyright 2015 Marco Draeger
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0

extern crate docopt;
extern crate network;
extern crate regex;
extern crate rustc_serialize;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{ BufReader, BufRead };
use std::path::Path;

use network::NodeId;
use network::algorithms::{ breadth_first_search, depth_first_search, heap_dijkstra };
use network::compact_star::{ compact_star_from_edge_vec };

mod usage;
use usage::{ get_args, DEFAULT_PATTERN, DEFAULT_SKIP };

mod parse_text;
use parse_text::{ Edge, edges_from_file };

mod alg_runner;
use alg_runner::run_algorithm;

fn main() {
    let ref args = get_args();
    let pattern = &args.flag_pattern
        .as_ref()
        .unwrap_or(&DEFAULT_PATTERN.to_string())
        .clone();
    let skip = args.flag_skip.unwrap_or(DEFAULT_SKIP);
    let file_name = &args.arg_filename;
    let is_undirected = &args.flag_undirected;

    let mut node_to_id: HashMap<String, NodeId> = HashMap::new();
    let mut edges: Vec<Edge> = Vec::new();

    edges_from_file(Path::new(file_name), 
                    pattern, 
                    is_undirected, 
                    skip, 
                    &mut node_to_id, 
                    &mut edges);
    let num_nodes = node_to_id.len();
    let compact_star = compact_star_from_edge_vec(num_nodes, &mut edges);

    run_algorithm(&compact_star, args, &node_to_id);
    let max_node_id = node_to_id.values().max().unwrap();
}

#[test]
fn test_breadth_first_search() {
    let mut test_edges = vec![(0,1,25.0,30.0),
    (0,2,35.0,50.0), 
        (1,3,15.0,40.0),
        (2,1,45.0,10.0),
        (3,2,15.0,30.0),
        (3,4,45.0,60.0),
        (4,2,25.0,20.0),
        (4,3,35.0,50.0)];
    let compact_star = compact_star_from_edge_vec(5, &mut test_edges);

    println!("breadth first search: {:?}", breadth_first_search(&compact_star, 0));
    println!("depth first search: {:?}", depth_first_search(&compact_star, 0));

    let mut test_edges = vec![
        (0,1,6.0,0.0),
        (0,2,4.0,0.0),
        (1,2,2.0,0.0),
        (1,3,2.0,0.0),
        (2,3,1.0,0.0),
        (2,4,2.0,0.0),
        (3,5,7.0,0.0),
        (4,3,1.0,0.0),
        (4,5,3.0,0.0)];
    let compact_star = compact_star_from_edge_vec(6, &mut test_edges);
    println!("dijkstra (with heap): {:?}", heap_dijkstra(&compact_star, 0));
}

