use std::thread::sleep;
use std::time::Duration;

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode},
};
use crate::direction::Direction;

mod direction;
mod snake;
mod food;
mod util;
mod constant;
mod test;

mod helper;

fn main() {
    enable_raw_mode().unwrap();
    // 1.创建地图
    let mut map = helper::create_map();
    // 2.创建蛇
    let mut snake = helper::create_snake();
    // 3.创建食物
    let mut food = helper::create_food();

    let mut current_direction = Direction::RIGHT;

    loop {
        util::clear_screen();
        // 将蛇加入地图
        helper::add_snake_to_map(&snake, &mut map);
        // 将食物加入地图
        helper::add_food_to_map(&food, &mut map);
        // 随机生成食物
        helper::random_new_food(&mut food, &snake, &mut map);
        // 打印地图、蛇、食物
        helper::print_map(map);
        current_direction = helper::input_control(&mut snake, &mut food, &mut map, current_direction);
        // 判断游戏是否结束
        if helper::is_game_over(&snake) {
            println!("游戏结束！！！！");
            break;
        }
        sleep(Duration::from_millis(snake.speed));
    }
    disable_raw_mode().unwrap();
}