use std::io::Read;
use std::process::Command;
use std::thread::{current, sleep};
use std::time::Duration;
use device_query::{DeviceEvents, DeviceQuery, DeviceState, Keycode};

const WIDTH: usize = 50;
const HEIGHT: usize = 10;

fn main() {

    // 1.创建地图
    let mut map_vec = create_map();
    // 2.创建蛇
    let mut snake = create_snake();
    let device_state = DeviceState::new();

    // let _ = device_state.on_key_down(|key| {
    //     println!("Keyboard key down: {:#?}", key);
    // });
    let mut current_direction = Direction::RIGHT;
    loop {
        clear_screen();
        // 3.将蛇加入地图
        add_snake_to_map(&snake, &mut map_vec);
        // 打印地图与蛇
        print_map(map_vec);
        current_direction = input_control(&mut snake, &mut map_vec, current_direction, &device_state);
        // move_snake(&mut snake, &mut map_vec, current_direction);

        sleep(Duration::from_millis(snake.speed));
    }

    //Command::new("cmd.exe").arg("/c").arg("pause").status().expect("clear error!");
}

#[derive(Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

/// 创建蛇
fn create_snake() -> Snake {
    let head = [5, 6];
    let speed = 1000;
    let body = vec![[5, 5], [5, 4]];
    return Snake { head, speed, body };
}

/// 将蛇加入地图
fn add_snake_to_map(snake: &Snake, map: &mut [[&'static str; WIDTH]; HEIGHT]) {
    let head = snake.head;
    map[head[0]][head[1]] = "●";
    for i in snake.body.iter() {
        map[i[0]][i[1]] = "▣";
    }
}

/// 移动蛇
fn move_snake(snake: &mut Snake, map: &mut [[&'static str; WIDTH]; HEIGHT], direction: Direction) {
    let before_head = snake.head;

    match direction {
        Direction::UP => {
            // 蛇头往前移动一格，以前的蛇头变成蛇身的位置
            map[before_head[0]][before_head[1]] = "▣";
            snake.head[0] -= 1;
            map[before_head[0]][before_head[1]] = "●";
        }
        Direction::DOWN => {
            // 蛇头往前移动一格，以前的蛇头变成蛇身的位置
            map[before_head[0]][before_head[1]] = "▣";
            snake.head[0] += 1;
            map[before_head[0]][before_head[1]] = "●";
        }
        Direction::LEFT => {
            // 蛇头往前移动一格，以前的蛇头变成蛇身的位置
            map[before_head[0]][before_head[1]] = "▣";
            snake.head[1] -= 1;
            map[before_head[0]][before_head[1]] = "●";
        }
        Direction::RIGHT => {
            // 蛇头往前移动一格，以前的蛇头变成蛇身的位置
            map[before_head[0]][before_head[1]] = "▣";
            snake.head[1] += 1;
            map[before_head[0]][before_head[1]] = "●";
        }
    }
    // 蛇身去掉最后一个元素
    if !snake.body.is_empty() {
        let snake_foot = snake.body.remove(snake.body.len() - 1);
        map[snake_foot[0]][snake_foot[1]] = " ";
    }
    // 插入蛇身第一个元素
    snake.body.insert(0, before_head);
}

/// 游戏是否结束
fn is_game_over() {}

/// 蛇
struct Snake {
    // ● 蛇头
    head: [usize; 2],
    // 速度
    speed: u64,
    // 蛇身
    body: Vec<[usize; 2]>,
}

/// 打印地图
fn print_map(map: [[&'static str; WIDTH]; HEIGHT]) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!("{}", map[i][j]);
        }
        println!();
    }
}

/// 创建地图
fn create_map() -> [[&'static str; WIDTH]; HEIGHT] {
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
                 map: &mut [[&'static str; WIDTH]; HEIGHT],
                 direction: Direction,
                 device_state: &DeviceState) -> Direction {
    // ss
    let mut dir = direction;
    // match control {
    //     'w' => {
    //         dir = Direction::UP;
    //         move_snake(snake, map, Direction::UP);
    //     }
    //     's' => {
    //         dir = Direction::DOWN;
    //         move_snake(snake, map, Direction::DOWN);
    //     }
    //     'a' => {
    //         dir = Direction::LEFT;
    //         move_snake(snake, map, Direction::LEFT);
    //     }
    //     'd' => {
    //         dir = Direction::RIGHT;
    //         move_snake(snake, map, Direction::RIGHT);
    //     }
    //     _ => {
    //         move_snake(snake, map, direction);
    //     }
    // }
    let keycodes = device_state.get_keys();
    if !keycodes.is_empty() {
        let keycode = keycodes.get(0);
        if let Some(key) = keycode {
            match key {
                Keycode::A => {
                    dir = Direction::LEFT;
                    move_snake(snake, map, Direction::LEFT);
                }
                Keycode::D => {
                    dir = Direction::RIGHT;
                    move_snake(snake, map, Direction::RIGHT);
                }
                Keycode::S => {
                    dir = Direction::DOWN;
                    move_snake(snake, map, Direction::DOWN);
                }
                Keycode::W => {
                    dir = Direction::UP;
                    move_snake(snake, map, Direction::UP);
                }
                _ => {
                    move_snake(snake, map, direction);
                }
            }
        }
    } else {
        move_snake(snake, map, direction);
    }
    return dir;
    // println!("keys = {:?}", device_state.get_keys())
}



