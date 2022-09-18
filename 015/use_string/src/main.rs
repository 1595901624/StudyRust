fn main() {
    println!("\n***************1、字符串的追加**************");
    let mut string = String::from("hello ");

    string.push('r');
    println!("追加字符 push() -> {}", string);

    string.push_str("ust!");
    println!("追加字符串 push_str() -> {}", string);

    println!("\n***************2、字符串的插入**************");
    let mut insert_string = String::from("hello rust!");

    insert_string.insert(5, ',');
    println!("插入字符 insert() -> {}", insert_string);

    insert_string.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", insert_string);

    println!("\n***************3、字符串的连接（+）**************");
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";

    println!("连接字符串 + -> {}", result);

    // 疑问？
    // 执行下面的代码报错
    // println!("输出 string_append -> {}", string_append);
    println!("\n***************3、字符串的连接（format!）**************");
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);

    println!("\n***************4、字符串的替换（replace）**************");
    // replace
    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);

    // replacen
    let string_replacen = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replacen.replacen("rust", "RUST", 1);

    dbg!(new_string_replacen);

    // replace_range
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);

    println!("\n***************5、字符串的删除（delete）**************");
    // pop
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);

    // remove
    let word = "中";
    let ch = "1";
    println!("word 占 {} 个字节", std::mem::size_of_val(word));
    println!("ch 占 {} 个字节", std::mem::size_of_val(ch));

    let mut string_remove = String::from("测试remove方法");
    println!("string_remove 占 {} 个字节", std::mem::size_of_val(string_remove.as_str()));
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);

    // truncate
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);

    // clear
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);
}

