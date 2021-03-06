use docopt::Docopt;
use alg_runner::Algorithm;
use network::NodeId;

pub const DEFAULT_EPS: f64 = 1e-6;
pub const DEFAULT_BETA: f64 = 0.2;
pub const DEFAULT_PATTERN: &'static str = "^(?P<from>[[:alnum:]]+).(?P<to>[[:alnum:]]+)\\s+(?P<cost>\\d+.\\d+).*$";
pub const DEFAULT_SKIP: usize = 0;
pub const DEFAULT_START_ID: NodeId = 0;

const USAGE: &'static str = "
Network handling

Usage:
    test_network <algorithm> <filename> [options]
    test_network (-h | --help)
    test_network (-v | --version)

Options:
    -h --help             Show this screen.
    -v --version          Show version.
    --pattern=<p>         Rust regular expression for decoding the input file. Must specify P<from>, P<to>, P<cost>, P<capacity>. If cost or capacity are unspecified, they default to 0.0 respectively.
    --undirected          Whether the graph is undirected. If set, two arcs are added per line. Defaults to false.
    --skip=<s>            Number of header lines in the input file. Defaults to zero.
    --start-node=<name>   The node name from which to search in a search algorithm like Dijkstra, Breadth-First-Search, or Depth-First-Search. Defaults to the first parsed node name.
    --target-node=<name>  The node name to reach in a search algorithm like Dijkstra, Breadth-First-Search, or Depth-First-Search. In PageRank, the node name which rank we want to know. No default given.
    --use-heap            Whether to use a heap to process Dijkstra's shortest path algorithm.
    --beta=<beta>         For PageRank, the teleportation probability parameter. Must be a double value in [0.0, 1.0]. Defaults to 0.2.
    --eps=<eps>           For PageRank and other numeric algorithms, the convergence parameter. Defaults to 1e-6.
";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    pub arg_algorithm: Algorithm,
    pub arg_filename: String,
    pub flag_pattern: Option<String>,
    pub flag_undirected: bool,
    pub flag_skip: Option<usize>,
    pub flag_start_node: Option<String>,
    pub flag_target_node: Option<String>,
    pub flag_use_heap: bool,
    pub flag_beta: Option<f64>,
    pub flag_eps: Option<f64>,
}

pub fn get_args() -> Args {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    args
}
