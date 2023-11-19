use std::io::{Read, Write};

fn main() {

    // 1. 输出
    std::io::stdout().write_fmt(format_args!("123")).unwrap();
    // 立即输出
    // std::io::stdout().flush().unwrap();
    std::io::stdout().write(&[65, 66, 67]).unwrap();
    std::io::stderr().write(&[68, 69, 70]).unwrap();
    print!("hello");

    std::io::stdout().write_fmt(format_args!("123\n")).unwrap();
    // 换行符的ascii码是10
    std::io::stdout().write(&[65, 66, 67, 10]).unwrap();
    std::io::stderr().write(&[68, 69, 70, 10]).unwrap();
    println!("hello");

    // 2. 输入
    let mut s =  String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    println!("{s}");
}

struct MyReader;

impl Read for MyReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Ok(0)
    }
}

struct MyWriter;

impl Write for MyWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
