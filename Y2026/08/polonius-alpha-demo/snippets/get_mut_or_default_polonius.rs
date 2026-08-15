use std::collections::HashMap;
use std::hash::Hash;

fn get_mut_or_default<'r, K, V>(
    map: &'r mut HashMap<K, V>,
    key: K,
) -> &'r mut V
where
    K: Hash + Eq + Copy,
    V: Default,
{
    match map.get_mut(&key) {
        Some(value) => value,
        None => {
            map.insert(key, V::default());
            map.get_mut(&key).unwrap()
        }
    }
}
