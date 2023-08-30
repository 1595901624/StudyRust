//! 贪吃蛇辅助模块

use crossterm::event::KeyCode;
use rand::Rng;
use crate::{constant, util};
use crate::constant::Map;
use crate::direction::Direction;
use crate::food::Food;
use crate::snake::Snake;

/// 创建食物
pub fn create_food() -> Food {
    let position = [5, 8];
    return Food::new(position, false);
}

/// 创建蛇
pub fn create_snake() -> Snake {
    let head = [5, 6];
    let speed = 200;
    let body = vec![[5, 5], [5, 4]];
    return Snake::new(head, speed, body);
}

/// 将蛇加入地图
pub fn add_snake_to_map(snake: &Snake, map: &mut Map) {
    let head = snake.head;
    map[head[0]][head[1]] = "●";
    for i in snake.body.iter() {
        map[i[0]][i[1]] = "▣";
    }
}

/// 将食物加入地图
pub fn add_food_to_map(food: &Food, map: &mut Map) {
    if food.eat {
        // 食物如果被吃了，不要加到地图
        return;
    }
    let position = food.position;
    map[position[0]][position[1]] = "▣";
}

/// 随机生成食物,生成食物的时候需要避开蛇的身体
pub fn random_new_food(food: &mut Food, snake: &Snake, map: &mut Map) {
    if !food.eat {
        // 如果当前的食物没有被吃掉，则不生成新的食物
        return;
    }
    let head = snake.head;
    let body = &(snake.body);
    let (x, y) = loop {
        let mut rng = rand::thread_rng();
        // 避免靠近墙体生成食物（降低难度）
        let x = rng.gen_range(3..constant::HEIGHT - 3);
        let y = rng.gen_range(3..constant::WIDTH - 3);
        // 生成的元素不能与蛇重叠
        if head != [x, y] && !body.contains(&[x, y]) {
            break (x, y);
        }
    };
    map[x][y] = "▣";
    food.position = [x, y];
    food.eat = false;
}

/// 移动蛇
pub fn move_snake(snake: &mut Snake, food: &mut Food, map: &mut Map, direction: Direction) {
    let before_head = snake.head;
    // 是否吃到了食物
    let eat: bool;
    match direction {
        Direction::UP => {
            // 蛇头往前移动一格，以前的蛇头变成蛇身的位置
            snake.head[0] -= 1;
            eat = snake.head == food.position;
            map[before_head[0]][before_head[1]] = "▣";
            map[snake.head[0]][snake.head[1]] = "●";
        }
        Direction::DOWN => {
            // 蛇头往前移动一格，以前的蛇头变成蛇身的位置
            snake.head[0] += 1;
            eat = snake.head == food.position;
            map[before_head[0]][before_head[1]] = "▣";
            map[snake.head[0]][snake.head[1]] = "●";
        }
        Direction::LEFT => {
            // 蛇头往前移动一格，以前的蛇头变成蛇身的位置
            snake.head[1] -= 1;
            eat = snake.head == food.position;
            map[before_head[0]][before_head[1]] = "▣";
            map[snake.head[0]][snake.head[1]] = "●";
        }
        Direction::RIGHT => {
            // 先判断是否吃到了食物
            snake.head[1] += 1;
            eat = snake.head == food.position;
            // 蛇头往前移动一格，以前的蛇头变成蛇身的位置
            map[before_head[0]][before_head[1]] = "▣";
            map[snake.head[0]][snake.head[1]] = "●";
        }
    }
    // 蛇身去掉最后一个元素
    if !snake.body.is_empty() && !eat {
        let snake_foot = snake.body.remove(snake.body.len() - 1);
        map[snake_foot[0]][snake_foot[1]] = " ";
    }
    if eat {
        food.eat = true;
    }
    // 插入蛇身第一个元素
    snake.body.insert(0, before_head);
}

/// 游戏是否结束
pub fn is_game_over(snake: &Snake) -> bool {
    let head = snake.head;
    // 如果蛇头触及到边界 触发游戏结束
    if head[0] == 0 || head[0] == constant::HEIGHT - 1
        || head[1] == 0 || head[1] == constant::WIDTH - 1 {
        return true;
    }
    let body_list = &snake.body;
    if body_list.is_empty() {
        return false;
    }
    // 蛇头触及到蛇的身体
    for body in body_list.iter() {
        if body[0] == head[0]
            && body[1] == head[1] {
            return true;
        }
    }
    return false;
}


/// 打印地图
pub fn print_map(map: Map) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!("{}", map[i][j]);
        }
        print!("\r\n");
    }
}

/// 创建地图
pub fn create_map() -> Map {
    let block = "■";
    let empty = " ";
    let mut map = [[empty; constant::WIDTH]; constant::HEIGHT];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            // 第一行和最后一行边界
            if i == 0 || i == map.len() - 1 {
                map[i][j] = block;
            }

            // 第一列和最后一列边界
            if j == 0 || j == map[i].len() - 1 {
                map[i][j] = block;
            }
        }
    }
    return map;
}

/// 输入控制
pub fn input_control(snake: &mut Snake,
                 food: &mut Food,
                 map: &mut Map,
                 direction: Direction) -> Direction {
    let mut dir = direction;
    let kb = util::keyboard_hit();
    if kb.is_some() {
        if let Some(keycode) = kb {
            match keycode {
                KeyCode::Char('a') | KeyCode::Char('A') => {
                    if dir != Direction::RIGHT {
                        dir = Direction::LEFT;
                        move_snake(snake, food, map, Direction::LEFT);
                    } else {
                        move_snake(snake, food, map, direction);
                    }
                }
                KeyCode::Char('d') | KeyCode::Char('D') => {
                    if dir != Direction::LEFT {
                        dir = Direction::RIGHT;
                        move_snake(snake, food, map, Direction::RIGHT);
                    } else {
                        move_snake(snake, food, map, direction);
                    }
                }
                KeyCode::Char('s') | KeyCode::Char('S') => {
                    if dir != Direction::UP {
                        dir = Direction::DOWN;
                        move_snake(snake, food, map, Direction::DOWN);
                    } else {
                        move_snake(snake, food, map, direction);
                    }
                }
                KeyCode::Char('w') | KeyCode::Char('W') => {
                    if dir != Direction::DOWN {
                        dir = Direction::UP;
                        move_snake(snake, food, map, Direction::UP);
                    } else {
                        move_snake(snake, food, map, direction);
                    }
                }
                KeyCode::Esc => {
                    std::process::exit(0);
                }
                _ => {
                    move_snake(snake, food, map, direction);
                }
            }
        }
    } else {
        move_snake(snake, food, map, direction);
    }
    return dir;
}
