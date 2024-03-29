#[cfg(test)]
mod tests {
    /// 格式化字符
    #[test]
    fn test_format() {
        // 1. 默认
        println!("hello world!");
        // 2. 通配符
        println!("今天是 {} 年 {} 月 {} 日", 2023, 6, 22);
        // 3. 通配符 + 位置下标
        println!("{0} * {1} = {2}, {2} / {1} = {0}", 5, 6, 30);
        // 4. 通配符 + 命名参数
        // 4.1
        println!("我的名字叫{name}, 今年{age}岁, 喜欢{hobby}", hobby = "打篮球", name = "张三", age = 18);
        // 4.2
        let name = "Lisi";
        let hobby = "唱歌";
        let age = 21;
        println!("我的名字叫{name}, 今年{age}岁, 喜欢{hobby}");
        // 5. 通配符 + 对齐方式  + 对齐宽度
        println!("二进制 {:b}", 31);
        println!("八进制 {:o}", 31);
        println!("十六进制(小写) {:x}", 31);
        println!("十六进制(大写) {:X}", 31);
        println!("科学计数(小写) {:e}", 100000_f32);
        println!("科学计数(大写) {:E}", 100000_f32);
        println!("输出指针  {:p}", "rust");
        println!("打印Debug视图 {:?}", ["A", "B", "C", "D"]);
        println!("输出标点符号 {:+}", 5);
        // 6 其它符号
        // 6.1 #
        println!("带前缀符二进制 {:#b}", 31);
        println!("带前缀符八进制 {:#o}", 31);
        println!("带前缀符十六进制(小写) {:#x}", 31);
        println!("带前缀符十六进制(大写) {:#X}", 31);
        println!("带换行和缩进的Debug打印 {:#?}", ["A", "B", "C", "D"]);

        // 6.2  > < ^
        println!("使用大于号右对齐 {:>6}{:>6}{:>6}", 1, 2, 3);
        println!("使用小于号左对齐 {:<6}{:<6}{:<6}", 1, 2, 3);
        println!("省略大于号右对齐 {:6}{:6}{:6}", 1, 2, 3);
        println!("居中对齐 {:^6}{:^6}{:^6}", 1, 2, 3);
        println!("填充任意字符居中对齐 {:-^6}{:*^6}{:1^6}", 1, 2, 3);

        // 6.3 0
        println!("二进制8位补零 {:08b}", 31);
        println!("八进制8位补零 {:08o}", 31);
        println!("十六进制16位补零 {:016b}", 31);

        //6.4
        println!("小数保留位数 {:.3} ", 0.01);
        println!("{}小数保留3位数 {:.*} --- 保留4位数 {:.*} ", 0.01, 3, 0.01, 4, 0.10);
    }
}