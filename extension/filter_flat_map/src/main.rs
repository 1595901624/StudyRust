use std::collections::HashMap;

fn main() {}

#[test]
fn filter_map_test() {
    let vec = vec![1, 2, 3, 4, 5, 6];
    let x: Vec<_> = vec.iter().filter_map(|item|
        if item % 2 == 0 {
            None
        } else {
            Some(item)
        }
    ).collect();
    dbg!(x);
}

#[test]
fn flat_map_test() {
    let mut map = HashMap::new();
    map.insert("animals", vec!["dog", "cat", "pig"]);
    map.insert("fruits", vec!["apple", "orange", "banana"]);
    map.insert("cities", vec!["Beijing", "Shanghai", "Guangzhou"]);

    let vec = vec!["animals", "fruits", "cities"];

    let result: Vec<&Vec<&str>> = vec.into_iter().flat_map(|item| {
        map.get(item)
    }).collect();

    dbg!(result);
}

#[test]
fn scan_test() {
    let vec = vec![1, 2, 3, 4, 5];
    let result: Vec<i32> = vec.iter().scan(0, |st, item| {
        let x = item * item;
        *st += x;
        if *st > 50 {
            None
        } else {
            Some(x)
        }
    }).collect();
    dbg!(result);
}