#[derive(Debug)]
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