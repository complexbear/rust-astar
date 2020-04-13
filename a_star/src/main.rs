/*
 * Implementation of A* algorithm for path finding
 */
use petgraph::algo::{astar};
use petgraph::dot::{Dot, Config};
use petgraph::graph::{Graph};

mod path;
use path::{PathNode};

fn main() {
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
    let mut deps = Graph::<PathNode, f32>::new();
    let a = deps.add_node(PathNode::new("A",1.0, 0, 0));
    let b = deps.add_node(PathNode::new("B",1.0, 1, 3));
    let c = deps.add_node(PathNode::new("C",1.0, 3, 5));
    let d = deps.add_node(PathNode::new("D",1.0, 7, 1));
    let e = deps.add_node(PathNode::new("E",1.0, 10, 3));

    let edges = [
        (a, b), (b, c), (c, e),
        (a, d), (d, e)
    ];
    for e in edges.iter() {
        let a = deps.node_weight(e.0).unwrap();
        let b = deps.node_weight(e.1).unwrap();
        deps.add_edge(e.0, e.1, a.travel_cost(b));
    }

    // println!("{}", Dot::with_config(&deps, &[Config::EdgeNoLabel]));

    let (cost, path) = astar(
        &deps,
        a,
        |n| n == e,
        |e| *e.weight(),
        |_| 0.0
    ).unwrap();
    println!("{:?}", path);
    println!("{}", cost);
}
