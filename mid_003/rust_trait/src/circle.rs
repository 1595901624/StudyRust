use crate::graph_trait::Graph;

/// ε
struct Circle {
    // εεΎ
    radius: f64,
}

impl Graph for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * 3.14;
    }
}