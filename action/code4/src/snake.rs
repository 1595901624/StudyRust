/// 蛇
pub struct Snake {
    // ● 蛇头
    pub head: [usize; 2],
    // 速度
    pub speed: u64,
    // 蛇身
    pub body: Vec<[usize; 2]>,
}