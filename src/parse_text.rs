use std::collections::HashMap;
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

use network::{Capacity, Cost, NodeId};

/// Describes one edge (arc) in a network, regardless of actual network
/// implementation.
pub type Edge = (NodeId, NodeId, Cost, Capacity);

fn parse_pattern(p: &str) -> Regex {
    Regex::new(p).ok().expect("Couldn't compile pattern.")
}

fn parse_line(line: &str, regex: &Regex, node_to_id: &mut HashMap<String, NodeId>, next_node: &mut NodeId) -> Edge {

    let captures = regex.captures(line);
    let from_s = captures.as_ref()
                         .and_then(|cgroup| cgroup.name("from"))
                         .unwrap_or("");
    let to_s = captures.as_ref()
                       .and_then(|cgroup| cgroup.name("to"))
                       .unwrap_or("");
    let cost: Cost = captures.as_ref()
                             .and_then(|cgroup| cgroup.name("cost"))
                             .and_then(|cstring| cstring.parse().ok())
                             .unwrap_or(0.0);
    let cap: Capacity = captures.and_then(|cgroup| cgroup.name("cap"))
                                .and_then(|cstring| cstring.parse().ok())
                                .unwrap_or(0.0);

    let from = if node_to_id.contains_key(from_s) {
      node_to_id[from_s]
    } else {
      node_to_id.insert(from_s.to_string(), inc_node_counter(next_node));
      node_to_id[from_s]
    };
    let to = if node_to_id.contains_key(to_s) {
      node_to_id[to_s]
    } else {
      node_to_id.insert(to_s.to_string(), inc_node_counter(next_node));
      node_to_id[to_s]
    };

    (from, to, cost, cap)
}

fn inc_node_counter(next_node: &mut NodeId) -> NodeId {
  *next_node += 1;
  *next_node - 1
}

/// Read a list of edges from a file.
///
/// Every line has to match the pattern `pattern` and the number of header
/// lines is determined by the `skip` parameter.
///
/// The result is stored in a mutable vector with correct `Edge` type.
pub fn edges_from_file<P>(filename: P, pattern: &str, is_undirected: &bool, skip: usize, node_to_id: &mut HashMap<String,NodeId>, edges: &mut Vec<Edge>) 
        where P: AsRef<Path> {
    let regex = parse_pattern(pattern);
    let mut next_node: NodeId = 0;
    let f = BufReader::new(File::open(filename).ok().expect("Opening the file went bad."));

    for line in f.lines().skip(skip) {
        let l = match line {
            Ok(l) => l,
            Err(_) => return
        };
        let (from, to, cost, cap) = parse_line(&l, &regex, node_to_id, &mut next_node);
        edges.push((from, to, cost, cap));
        if *is_undirected {
          edges.push((to, from, cost, cap));
        }
    }
}

#[test]
fn test_pattern_match() {
    let pattern = "^(?P<from>[[:alnum:]]+).(?P<to>[[:alnum:]]+)\\s+(?P<cost>\\d+.\\d+).*$";
    let regex = parse_pattern(pattern);
    let to_match = "nW0770230N0388068.nW0770230N0388073   000.0345 065 11 {DC}";
    assert!(regex.is_match(to_match));
    assert_eq!(parse_pattern(r"^([[:alnum:]]+)$").captures("nW0770230N0388068").unwrap().at(1), Some("nW0770230N0388068"));
    let caps = regex.captures(to_match).unwrap();
    assert_eq!(Some("nW0770230N0388068"), caps.at(1)); 
    assert_eq!(Some("nW0770230N0388073"), caps.at(2)); 
    assert_eq!(Some("000.0345"), caps.at(3)); 
    
    for sub_named in caps.iter_named() {
      match sub_named {
        ("from", from) => assert_eq!(Some("nW0770230N0388068"), from),
        ("to", to) => assert_eq!(Some("nW0770230N0388073"), to),
        ("cost", cost) => assert_eq!(Some("000.0345"), cost),
        ("cap", cap) => assert_eq!(None, cap),
        (_, _) => assert!(false),
      }
    }
}
