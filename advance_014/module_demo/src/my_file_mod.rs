use crate::util;
// 别名
use crate::util as my_util;

pub fn test() {
    util::my_println("hello world!");
    my_util::my_println("hello world!");
}