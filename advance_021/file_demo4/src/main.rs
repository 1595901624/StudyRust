use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

fn main() {
    println!("Hello, world!");
}

#[test]
fn open_option_test() {
    // 创建文件
    // 必须使用写入或追加访问权限打开文件才能创建新文件。
    let open_option = OpenOptions::new().write(true).create(true).open("test.txt");
    println!("{:?}", open_option);

    // 创建新文件
    // 必须使用写入或追加访问权限打开文件才能创建新文件。
    let open_option = OpenOptions::new().write(true).create_new(true).open("test2.txt");
    println!("{:?}", open_option);

    // 追加
    let open_option = OpenOptions::new().append(true).open("test2.txt");
    println!("{:?}", open_option);

    // 读文件
    let open_option = OpenOptions::new().read(true).open("test2.txt");
    println!("{:?}", open_option);

    // 写文件
    let open_option = OpenOptions::new().write(true).open("test2.txt");
    println!("{:?}", open_option);

    // 截断
    let open_option = OpenOptions::new().write(true).truncate(true).open("test2.txt");
    println!("{:?}", open_option);
}

/// 读取操作
#[test]
fn read_test() {
    // let create_result = File::create("create.txt").unwrap();
    // std::fs::write()
    // println!("{:?}", create_result);
    let mut file = OpenOptions::new().read(true).open("read_demo.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    println!("{:?}", s);
}

/// 写入操作
#[test]
fn write_demo() {
    let mut file = OpenOptions::new().write(true).create(true).open("write_demo.txt").unwrap();
    let write_result = file.write_all(b"hello world");
    println!("{:?}", write_result);
}

#[test]
fn buf_writer() {
    let file = OpenOptions::new().write(true).create(true).open("write_demo.txt").unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(b"Hello, Rust!").expect("write error");
    // 注意：不要忘记刷新缓冲区，以确保所有数据都被写入文件
    writer.flush().expect("flush error");
}


#[test]
fn buf_reader() {
    let file = OpenOptions::new().read(true).open("read_demo.txt").unwrap();
    let reader = BufReader::new(file);
    // 按行读取
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}