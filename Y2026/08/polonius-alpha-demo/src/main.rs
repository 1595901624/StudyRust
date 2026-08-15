use std::collections::HashMap;
use std::hash::Hash;

fn get_mut_or_default<K, V>(map: &mut HashMap<K, V>, key: K) -> &mut V
where
    K: Hash + Eq,
    V: Default,
{
    // Entry API 只针对这个 key 做一次查找
    map.entry(key).or_default()
}

fn main() {
    let mut scores = HashMap::new();
    scores.insert("rust", 100);

    // 已存在的 key 返回原有值的可变引用
    *get_mut_or_default(&mut scores, "rust") += 1;

    // 不存在的 key 插入默认值后返回可变引用
    *get_mut_or_default(&mut scores, "polonius") += 1;

    println!("rust: {}", scores["rust"]);
    println!("polonius: {}", scores["polonius"]);
}
