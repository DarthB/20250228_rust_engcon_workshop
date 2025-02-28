use std::fmt::Display;

pub struct DistillationColumn {
    trays: i32,
    feed_place: i32,
    reflux_ratio: f32,
    distiliate_to_feed_ratio: f32,
}

// implementation of the trait default
impl Default for DistillationColumn {
    fn default() -> Self {
        Self::new()
    }
}

impl DistillationColumn {
    pub fn new() -> Self {
        Self {
            trays: 20,
            feed_place: 10,
            reflux_ratio: 1.5,
            distiliate_to_feed_ratio: 0.9,
        }
    }

    pub fn energy(&self) -> f32 {
        1.0 / self.reflux_ratio
    }

    pub fn cost(&self, c0: f32, coeff_a: f32, coeff_b: f32) -> f32 {
        c0 + coeff_a * self.trays as f32 + coeff_b * self.reflux_ratio
    }
}

impl Display for DistillationColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DistillationColumn {{ trays: {}, feed_place: {}, reflux_ratio: {}, distiliate_to_feed_ratio: {} }}\n\t--> Energy: {}, Costs: {}",
            self.trays,
            self.feed_place,
            self.reflux_ratio,
            self.distiliate_to_feed_ratio,
            self.energy(),
            self.cost(20.0, 20.0, 10.0)
        )
    }
}

fn main() {
    let ds = DistillationColumn::default();
    println!("{}", ds);
}
