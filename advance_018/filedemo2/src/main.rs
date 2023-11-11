use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

fn main() {
    println!("Hello, world!");
}

#[test]
fn create_path() {
    let path1 = Path::new("src").join("main.rs");
    println!("path: {:?}", path1);

    let path2 = PathBuf::new().join("src").join("main.rs");
    println!("path: {:?}", path2);
}

#[test]
fn test_common_function() {
    let path = Path::new("C:\\test\\hello.rs");
    println!("path 路径: {}", path.display());

    let path_buf = PathBuf::from("C:\\test\\hello.rs");
    println!("path_buf 路径: {}", path_buf.display());
}

#[test]
fn test_path_function() {
    // Path
    let path: &Path = Path::new("C:\\test\\hello.rs");
    // 完整文件名称
    let file_name: Option<&OsStr> = path.file_name();
    // 文件后缀
    let file_extension: Option<&OsStr> = path.extension();
    // 文件名（不包含后缀）
    let file_stem: Option<&OsStr> = path.file_stem();
    // 父目录
    let parent: Option<&Path> = path.parent();
    // 设置扩展名（实际使用的是 PathBuf.set_extension 方法）
    let new_extension: PathBuf = path.with_extension("txt");

    println!("path: {:?}", path);
    println!("file_name: {:?}", file_name);
    println!("file_extension: {:?}", file_extension);
    println!("file_stem: {:?}", file_stem);
    println!("parent: {:?}", parent);
    println!("new_extension: {:?}", new_extension);
}

#[test]
fn test_path_buf_function() {
    // PathBuf
    let path_buf: PathBuf = PathBuf::from("C:\\test\\hello.rs");
    // 完整文件名称
    let file_name: Option<&OsStr> = path_buf.file_name();
    // 文件后缀
    let file_extension: Option<&OsStr> = path_buf.extension();
    // 文件名（不包含后缀）
    let file_stem: Option<&OsStr> = path_buf.file_stem();
    // 父目录
    let parent: Option<&Path> = path_buf.parent();
    // 设置扩展名（实际使用的是 PathBuf.set_extension 方法）
    let new_extension: PathBuf = path_buf.with_extension("txt");

    println!("file_name: {:?}", file_name);
    println!("file_extension: {:?}", file_extension);
    println!("file_stem: {:?}", file_stem);
    println!("parent: {:?}", parent);
    println!("new_extension: {:?}", new_extension);
}

#[test]
fn test_other_function() {
    // 设置扩展名（实际使用的是 PathBuf.set_extension 方法）
    let path: PathBuf = PathBuf::from("C:\\test\\hello.rs");
    let new_extension: PathBuf = path.with_extension("txt");
    println!("来自PathBuf => new_extension: {:?}", new_extension);

    // 设置扩展名（实际使用的是 PathBuf.set_extension 方法）
    let path: &Path = Path::new("C:\\test\\hello.rs");
    let new_extension: PathBuf = path.with_extension("txt");
    println!("来自Path => new_extension: {:?}", new_extension);
}

#[test]
fn test_path_param() {
    let str = "C:\\test\\hello.rs";
    let string = String::from(str);
    let os_str = OsStr::new(str);
    let os_string = OsString::from(str);
    let path = Path::new(str);
    let path_buf = PathBuf::from(str);

    get_path(str);
    get_path(string);
    get_path(os_str);
    get_path(os_string);
    get_path(path);
    get_path(path_buf);
}

#[test]
fn test_os_str_param() {
    let str = "C:\\test\\hello.rs";
    let string = String::from(str);
    let os_str = OsStr::new(str);
    let os_string = OsString::from(str);
    let path = Path::new(str);
    let path_buf = PathBuf::from(str);

    get_os_str(str);
    get_os_str(string);
    get_os_str(os_str);
    get_os_str(os_string);
    get_os_str(path);
    get_os_str(path_buf);
}

#[test]
fn test_str_param() {
    let str = "C:\\test\\hello.rs";
    let string = String::from(str);

    get_str(str);
    get_str(string);
}

#[test]
fn test_u8_param() {
    let str = "C:\\test\\hello.rs";
    let string = String::from(str);

    get_u8(str);
    get_u8(string);
}


fn get_path<P>(p: P) where P: AsRef<Path> {
    let path: &Path = p.as_ref();
    println!("path: {:?}", path);
}

fn get_os_str<OS>(os: OS) where OS: AsRef<OsStr> {
    let path: &OsStr = os.as_ref();
    println!("os_str: {:?}", path);
}

fn get_str<S>(s: S) where S: AsRef<str> {
    let str: &str = s.as_ref();
    println!("str: {:?}", str);
}

fn get_u8<U>(u: U) where U: AsRef<[u8]> {
    let str: &[u8] = u.as_ref();
    println!("[u8]: {:?}", str);
}