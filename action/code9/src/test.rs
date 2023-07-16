/// 格式化时间
#[test]
pub fn test_time() {
    use chrono::Local;

    let current_time = Local::now();
    let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    println!("{}", formatted_time);
}

#[test]
pub fn test_dir() {
    use std::path::Path;

    // 创建单级文件目录
    std::fs::create_dir("lanqiao").unwrap();

    // 创建多级文件目录
    std::fs::create_dir_all("one/two/three").unwrap();

    // 创建文件并写入内容
    let path = Path::new("lanqiao");
    let file_path = path.join("test.txt");
    std::fs::write(file_path, "Hello, Rust!").unwrap();

    // 重命名文件 test.txt
    std::fs::rename("lanqiao/test.txt", "lanqiao/test_new.txt").unwrap();

    // 复制文件 test_new.txt 到 one/two/three 目录下
    std::fs::copy("lanqiao/test_new.txt", "one/two/three/test_new.txt").unwrap();

    // 删除文件 lanqiao/test_new.txt
    std::fs::remove_file("lanqiao/test_new.txt").unwrap();

    // 删除单级文件目录
    std::fs::remove_dir("lanqiao").unwrap();
    // 删除多级文件目录
    std::fs::remove_dir_all("one").unwrap();
}

#[test]
pub fn test_file_option() {
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    // 写文件
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("file_option.txt")
        .expect("Failed to open file");

    let contents = "Hello, Rust!";
    file.write(contents.as_bytes()).expect("Failed to write to file");

    // 读文件
    let mut file = OpenOptions::new()
        .read(true)
        .open("file_option.txt")
        .expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    println!("{}", contents);

    // 删除文件
    std::fs::remove_file("file_option.txt").unwrap();
}