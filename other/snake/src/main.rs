use std::io::Read;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

const WIDTH: usize = 80;
const HEIGHT: usize = 10;

fn main() {
    // 1.创建地图
    let mut map_vec = create_map();
    // 2.创建蛇
    let mut snake = create_snake();

    loop {
        clear_screen();
        // 3.将蛇加入地图
        add_snake_to_map(&snake, &mut map_vec);
        // 打印地图与蛇
        print_map(map_vec);
        // input_control(&snake, &mut map_vec);
        move_snake(&mut snake, &mut map_vec);

        sleep(Duration::from_millis(snake.speed));
    }


    //Command::new("cmd.exe").arg("/c").arg("pause").status().expect("clear error!");
}

/// 创建蛇
fn create_snake() -> Snake {
    let head = [5, 6];
    let speed = 1000;
    return Snake { head, speed };
}

/// 将蛇加入地图
fn add_snake_to_map(snake: &Snake, map: &mut [[&'static str; WIDTH]; HEIGHT]) {
    let head = snake.head;
    map[head[0]][head[1]] = "●";
}

/// 移动蛇
fn move_snake(snake: &mut Snake, map: &mut [[&'static str; WIDTH]; HEIGHT]) {
    map[snake.head[0]][snake.head[1]] = " ";
    snake.head[1] += 1;
    map[snake.head[0]][snake.head[1]] = "●";
}

/// 游戏是否结束
fn is_game_over() {

}

/// 蛇
struct Snake {
    // ● 蛇头
    head: [usize; 2],
    // 速度
    speed: u64,
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

///
fn input_control(snake: &Snake, map: &mut [[&'static str; WIDTH]; HEIGHT]) {
    let mut input: [u8; 1] = [0];
    std::io::stdin().read(&mut input).expect("input error!");
    // println!("{}", input[0]);
    let control = char::from(input[0]);
    match control {
        'w' => {
            println!("上");
        }
        's' => {
            println!("下");
        }
        'a' => {
            println!("左");
        }
        'd' => {
            println!("右");
        }
        _ => {}
    }
}



