use std::collections::HashMap;
use std::hash::Hash;

fn get_mut_or_default<K, V>(
    map: &mut HashMap<K, V>,
    key: K,
) -> &mut V
where
    K: Hash + Eq,
    V: Default,
{
    map.entry(key).or_default()
}
