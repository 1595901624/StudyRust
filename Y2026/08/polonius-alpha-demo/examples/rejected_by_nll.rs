use std::collections::HashMap;
use std::hash::Hash;

fn get_mut_or_default<'r, K, V>(map: &'r mut HashMap<K, V>, key: K) -> &'r mut V
where
    K: Hash + Eq + Copy,
    V: Default,
{
    // Some 分支返回第一次查找到的可变引用
    match map.get_mut(&key) {
        Some(value) => value,
        None => {
            // None 分支重新借用 map 插入默认值
            map.insert(key, V::default());
            map.get_mut(&key).unwrap()
        }
    }
}

fn main() {
    let mut scores: HashMap<&str, i32> = HashMap::new();
    *get_mut_or_default(&mut scores, "polonius") += 1;
    println!("polonius: {}", scores["polonius"]);
}
