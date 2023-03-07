use std::process::Command;
use std::thread::{sleep};
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};
use rand::Rng;

const WIDTH: usize = 50;
const HEIGHT: usize = 10;

type Map = [[&'static str; WIDTH]; HEIGHT];

fn main() {

    // 1.创建地图
    let mut map = create_map();
    // 2.创建蛇
    let mut snake = create_snake();
    // 3.创建食物
    let mut food = create_food();

    let device_state = DeviceState::new();

    // let _ = device_state.on_key_down(|key| {
    //     println!("Keyboard key down: {:#?}", key);
    // });
    let mut current_direction = Direction::RIGHT;
    loop {
        clear_screen();
        // 将蛇加入地图
        add_snake_to_map(&snake, &mut map);
        // 将食物加入地图
        add_food_to_map(&food, &mut map);
        // 随机生成食物
        random_new_food(&mut food, &snake, &mut map);
        // 打印地图、蛇、食物
        print_map(map);
        current_direction = input_control(&mut snake, &mut food, &mut map, current_direction, &device_state);
        // move_snake(&mut snake, &mut map, current_direction);
        // 判断游戏是否结束
        if is_game_over(&snake) {
            println!("游戏结束！！！！");
            break;
        }
        sleep(Duration::from_millis(snake.speed));
    }
    Command::new("cmd.exe").arg("/c").arg("pause").status().expect("clear error!");
}

/// 将食物加入地图
fn add_food_to_map(food: &Food, map: &mut Map) {
    if food.eat {
        // 食物如果被吃了，不要加到地图
        return;
    }
    let position = food.position;
    map[position[0]][position[1]] = "▣";
}

/// 随机生成食物
/// 随机算法还有点儿小bug
fn random_new_food(food: &mut Food, snake: &Snake, map: &mut Map) {
    if !food.eat {
        // 如果当前的食物没有被吃掉，则不生成新的食物
        return;
    }
    // let head = snake.head;
    // let body = &(snake.body);
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(1..HEIGHT - 1);
    let y = rng.gen_range(1..WIDTH - 1);
    map[x][y] = "▣";
    food.position = [x, y];
    food.eat = false;
}

#[derive(Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

/// 创建食物
fn create_food() -> Food {
    let position = [5, 8];
    return Food { position, eat: false };
}

/// 创建蛇
fn create_snake() -> Snake {
    let head = [5, 6];
    let speed = 1000;
    let body = vec![[5, 5], [5, 4]];
    return Snake { head, speed, body };
}

/// 将蛇加入地图
fn add_snake_to_map(snake: &Snake, map: &mut Map) {
    let head = snake.head;
    map[head[0]][head[1]] = "●";
    for i in snake.body.iter() {
        map[i[0]][i[1]] = "▣";
    }
}

/// 移动蛇
fn move_snake(snake: &mut Snake, food: &mut Food, map: &mut Map, direction: Direction) {
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
fn is_game_over(snake: &Snake) -> bool {
    let head = snake.head;
    // 如果蛇头触及到边界 触发游戏结束
    if head[0] == 0 || head[0] == HEIGHT - 1
        || head[1] == 0 || head[1] == WIDTH - 1 {
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

/// 蛇
struct Snake {
    // ● 蛇头
    head: [usize; 2],
    // 速度
    speed: u64,
    // 蛇身
    body: Vec<[usize; 2]>,
}

/// 食物
struct Food {
    position: [usize; 2],
    eat: bool,
}

/// 打印地图
fn print_map(map: Map) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!("{}", map[i][j]);
        }
        println!();
    }
}

/// 创建地图
fn create_map() -> Map {
    let block = "■";
    let empty = " ";
    let mut map = [[empty; WIDTH]; HEIGHT];
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

/// 清空屏幕
fn clear_screen() {
    Command::new("cmd.exe").arg("/c").arg("cls").status().expect("clear error!");
}

/// 键盘控制
fn input_control(snake: &mut Snake,
                 food: &mut Food,
                 map: &mut Map,
                 direction: Direction,
                 device_state: &DeviceState) -> Direction {
    let mut dir = direction;
    let keycodes = device_state.get_keys();
    if !keycodes.is_empty() {
        let keycode = keycodes.get(0);
        if let Some(key) = keycode {
            match key {
                Keycode::A => {
                    dir = Direction::LEFT;
                    move_snake(snake, food, map, Direction::LEFT);
                }
                Keycode::D => {
                    dir = Direction::RIGHT;
                    move_snake(snake, food, map, Direction::RIGHT);
                }
                Keycode::S => {
                    dir = Direction::DOWN;
                    move_snake(snake, food, map, Direction::DOWN);
                }
                Keycode::W => {
                    dir = Direction::UP;
                    move_snake(snake, food, map, Direction::UP);
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
    // println!("keys = {:?}", device_state.get_keys())
}



