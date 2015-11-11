#![crate_name="network"]
#![crate_type="lib"]

pub mod compact_star;
pub mod algorithms;
mod collections;

pub type DoubleVec = Vec<f64>;
pub type NodeId    = u32;
pub type NodeVec   = Vec<NodeId>;

pub trait Network {
    /// Returns a vec of adjecent nodes, identified by their id
    fn adjacent(&self, i: NodeId) -> Vec<NodeId>;
    fn cost(&self, from: NodeId, to: NodeId) -> Option<f64>;
    fn capacity(&self, from: NodeId, to: NodeId) -> Option<f64>;
    fn num_nodes(&self) -> usize;
    fn num_arcs(&self) -> usize;
    /// Returns an invalid node id to be used as default/ stop value.
    /// In Ahuja, Magnati, Orlin: "Network Flows", this is 0, but
    /// that would mean to have all indexing one-based and this feels
    /// too unnatural
    fn invalid_id(&self) -> NodeId;
}
