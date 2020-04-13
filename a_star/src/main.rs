/*
 * Implementation of A* algorithm for path finding
 */
use petgraph::dot::{Dot, Config};

mod path;
use path::{PathNode, create_graph};

fn main() {
    let a = PathNode::new("A", 1.0, 0, 0);
    let b = PathNode::new("B", 2.0, 6, 8);

    println!("node: {}", a);
    println!("node: {}", b);
    println!("travel from a to b: {}", a.travel_cost(&b));

    let g = create_graph();
    println!("{}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
}
