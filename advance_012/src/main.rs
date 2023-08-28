use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Read;

fn main() {
    // 1. result
    let result = test_result(101);
    match result {
        Ok(value) => println!("Value: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    // 2. result alias
    let result = test_result_alias(101);
    match result {
        Ok(value) => println!("Value: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    // 3. 自定义错误类型
    let test_result = test_my_error();
    match test_result {
        Ok(_) => {}
        Err(e) => {
            println!("MyError: {:?}", e.to_string());
        }
    }

    // 4. Error 的传递
    let test_result = test_error_propagation();
    match test_result {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {:?}", e.to_string());
        }
    }

    // 5. 不同类型的 Error 的传递
    let test_result = test_error_propagation_different();
    match test_result {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {:?}", e.to_string());
        }
    }

    // 6. 其它处理方式
    // 6.1 unwrap
    let value = "10".parse::<i32>().unwrap_or(0);
    println!("Value: {}", value);

    // 6.2 unwrap_or
    let value = "10x".parse::<i32>().unwrap_or(0);
    println!("Value: {}", value);

    // 6.3 unwrap_or_else
    let value = "10x".parse::<i32>().unwrap_or_else(|e| {
        println!("Error: {:?}", e.to_string());
        0
    });
    println!("Value: {}", value);

    // 6.4 expect
    let value = "10x".parse::<i32>().expect("解析失败");
    println!("Value: {}", value);

    // 6.5 忽略错误
    let _ = "10x".parse::<i32>();
}

fn test_result(input: i32) -> Result<i32, String> {
    if input <= 100 {
        return Ok(input);
    }
    return Err(String::from("输入的值大于100"));
}

// 2. result alias
fn test_result_alias(input: i32) -> std::io::Result<i32> {
    if input <= 100 {
        return Ok(input);
    }
    return Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "输入的值大于100",
    ));
}


// 3. 自定义错误类型
#[derive(Debug)]
struct MyError;

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "这是一个自定义错误类型")
    }
}

impl std::error::Error for MyError {}

fn test_my_error() -> Result<(), MyError> {
    Err(MyError)
}

// 4. Error 的传递
fn test_error_propagation() -> std::io::Result<()> {
    // 这是一个读写文件的例子
    // 打开文件可能会失败，如果出现错误，会直接返回错误
    let mut f = File::open("C:/Users/xxx/Desktop/hello.txt")?;
    let mut s = String::new();
    // 读取文件可能会失败，如果出现错误，会直接返回错误
    let size = f.read_to_string(&mut s)?;
    println!("文件大小: {}", size);
    // 没有错误，返回Ok
    Ok(())
}

// 5. 不同类型的 Error 的传递
fn test_error_propagation_different() -> Result<(), Box<dyn std::error::Error>> {
    // 打开文件可能会失败(std::io::Error)，如果出现错误，会直接返回错误
    let mut f = File::open("C:/Users/xxx/Desktop/hello.txt")?;
    let mut s = String::new();
    // 读取文件可能会失败(std::io::Error)，如果出现错误，会直接返回错误
    let size = f.read_to_string(&mut s)?;
    println!("文件大小: {}", size);
    // 将字符串转换为数字可能会失败(ParseIntError)，如果出现错误，会直接返回错误
    let value = s.parse::<i32>()?;
    println!("文件内容: {}", value);
    // 触发自定义错误(MyError)，如果出现错误，会直接返回错误
    let my_error = test_my_error()?;
    println!("自定义错误: {:?}", my_error);
    // 没有错误，返回Ok
    Ok(())
}
