/*
 * Implementation of A* algorithm for path finding
 */
mod path;
use path::PathNode;

fn main() {
    let a = PathNode::new(1.0, 0, 0);
    let b = PathNode::new(2.0, 6, 8);

    println!("node: {:?}", a);
    println!("node: {:?}", b);
    println!("travel from a to b: {}", a.travel_cost(&b));
}
