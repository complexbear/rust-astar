use std::fmt;
use petgraph::Graph;

#[derive(Debug)]
#[derive(Default)]
pub struct PathNode {
    name: String,
    cost_weight: f32,
    pos_x: i32,
    pos_y: i32
}

impl PathNode {
    pub fn new(n: &str, w: f32, x: i32, y: i32) -> PathNode {
        PathNode {
            name: n.to_string(),
            cost_weight: w,
            pos_x: x,
            pos_y: y
        }
    }
    pub fn travel_cost(&self, target: &PathNode) -> f32 {
        // calc distance between nodes and multiply by cost weighting
        let z = (
            (target.pos_x - self.pos_x).pow(2).abs() +
            (target.pos_y - self.pos_y).pow(2).abs()
        ) as f32;
        return z.sqrt();
    }
}

impl fmt::Display for PathNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{} ({}:{})", self.name, self.cost_weight, self.pos_x, self.pos_y)
    }
}

pub fn create_graph() -> Graph<PathNode, PathNode> {
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
    return deps;
}
