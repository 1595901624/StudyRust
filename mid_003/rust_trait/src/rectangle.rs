use crate::graph_trait::Graph;

pub struct Rectangle {
    // 宽度
    pub(crate) width: f64,
    // 高度
    pub(crate) height: f64,
}

impl Graph for Rectangle {
    fn area(&self) -> f64 {
        return self.height * self.width;
    }
}
