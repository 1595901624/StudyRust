/// 食物
pub struct Food {
    pub position: [usize; 2],
    pub eat: bool,
}

impl Food {
    pub fn new(position: [usize; 2], eat: bool) -> Self {
        Self {
            position,
            eat,
        }
    }
}