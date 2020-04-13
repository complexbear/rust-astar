use std::fmt;
use trees::{tr, fr, Tree};

pub struct PathNode {
    cost_weight: f32,
    pos_x: i32,
    pos_y: i32
}

impl PathNode {
    pub fn new(w: f32, x: i32, y: i32) -> PathNode {
        PathNode {
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
        write!(f, "PathNode: {}, ({}:{})", self.cost_weight, self.pos_x, self.pos_y)
    }
}

pub fn create_tree() -> Tree<PathNode> {
    let n = tr(PathNode::new(1.0, 0, 0));
    return n;
}
