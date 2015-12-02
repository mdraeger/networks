#![crate_name="network"]
#![crate_type="lib"]

use std::f64::{ INFINITY, NEG_INFINITY };

pub mod compact_star;
pub mod algorithms;
mod collections;
mod heaps;

pub type DoubleVec = Vec<f64>;
pub type Capacity  = f64;
pub type Cost      = f64;
pub type NodeId    = u32;
pub type NodeVec   = Vec<NodeId>;

pub const INF: Cost = INFINITY;
pub const NEG_INF: Cost = NEG_INFINITY;

pub trait Network {
    /// Returns a vec of adjecent nodes, identified by their id
    fn adjacent(&self, i: NodeId) -> Vec<NodeId>;
    fn cost(&self, from: NodeId, to: NodeId) -> Option<Cost>;
    fn capacity(&self, from: NodeId, to: NodeId) -> Option<Capacity>;
    fn num_nodes(&self) -> usize;
    fn num_arcs(&self) -> usize;
    /// Returns an invalid node id to be used as default/ stop value.
    /// In Ahuja, Magnati, Orlin: "Network Flows", this is 0, but
    /// that would mean to have all indexing one-based and this feels
    /// too unnatural
    fn invalid_id(&self) -> NodeId;
    fn infinity(&self) -> Cost;
}
