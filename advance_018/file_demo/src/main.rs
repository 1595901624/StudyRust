use std::ffi::{OsStr, OsString};
use std::str::FromStr;

fn main() {
    let s1 = OsStr::new("hello");
    let s2 = s1.to_os_string();

    println!("{:?}", s1);
    println!("{:?}", s2);


}

