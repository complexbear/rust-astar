/*
 * Implementation of A* algorithm for path finding
 */
use petgraph::algo::{dijkstra};
use petgraph::dot::{Dot, Config};
use petgraph::Graph;

mod path;
use path::{PathNode};


fn main() {
    let a = PathNode::new("A", 1.0, 0, 0);
    let b = PathNode::new("B", 2.0, 6, 8);

    println!("node: {}", a);
    println!("node: {}", b);
    println!("travel from a to b: {}", a.travel_cost(&b));

    /*
    Basic layout of the paths in the map
    a = start
    e = end
    |         __ c(3,5)_______
    |        /                \
    |   b(1,3)                 e(10,3)
    |   /                     /
    |   |                d(7,1)
    a(0,0)______________/
    */
    let mut deps = Graph::<PathNode, PathNode>::new();
    let a = deps.add_node(PathNode::new("A",1.0, 0, 0));
    let b = deps.add_node(PathNode::new("B",1.0, 1, 3));
    let c = deps.add_node(PathNode::new("C",1.0, 3, 5));
    let d = deps.add_node(PathNode::new("D",1.0, 7, 1));
    let e = deps.add_node(PathNode::new("E",1.0, 10, 3));

    deps.extend_with_edges(&[
        (a, b), (b, c), (c, e),
        (a, d), (d, e)
    ]);

    println!("{}", Dot::with_config(&deps, &[Config::EdgeNoLabel]));

    let node_map = dijkstra(&deps, a.into(), Some(e.into()), |_| 1);
    println!("{:?}", node_map);
}
