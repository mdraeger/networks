use docopt::Docopt;

const USAGE: &'static str = "
Network handling

Usage:
    test_network <algorithm> <filename> --pattern=<p> [options]
    test_network (-h | --help)
    test_network (-v | --version)

Options:
    -h --help           Show this screen.
    -v --version        Show version.
    --pattern=<p>       Rust regular expression for decoding the input file. Must specifiy P<from>, P<to>, P<cost>, P<capacity>
    --use-heap          Whether to use a heap to process Dijkstra's shortest path algorithm.
    --undirected        Whether the graph is undirected. If set, two arcs are added per line.
    --beta=<beta>       For PageRank, the teleportation probability parameter. Must be a double value in [0.0, 1.0]
";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    arg_algorithm: Algorithm,
    arg_filename: String,
    pub flag_pattern: String,
    flag_undirected: bool,
    flag_use_heap: bool,
    flag_beta: Option<f64>,
}

#[derive(Debug, RustcDecodable)]
enum Algorithm { dijkstra, pagerank }

pub fn getArgs() -> Args {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
    args
}
