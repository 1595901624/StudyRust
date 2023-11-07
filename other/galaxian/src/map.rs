pub const MAP_HEIGHT: usize = 60;
pub const MAP_WIGTH: usize = 100;

pub const PLANE_HEIGHT: usize = 4;
pub const PLANE_WIGTH: usize = 10;

pub type Map = [[&'static str; MAP_WIGTH]; MAP_HEIGHT];

pub type PlaneMap = [[&'static str; PLANE_WIGTH]; PLANE_HEIGHT];

// 游戏地图
pub const MAP: Map = [["X"; MAP_WIGTH]; MAP_HEIGHT];
// 飞机地图
pub const PLANE_MAP: PlaneMap = [["X"; PLANE_WIGTH]; PLANE_HEIGHT];
