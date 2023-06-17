use crate::linked_list::LinkedList;

mod linked_list;
mod test;

fn main() {
    let mut list = LinkedList::new();
    // list.add(3);
    // list.add(4);
    // list.add(5);
    // list.add(6);
    // list.add(7);
    // list.add(8);
    //
    // list.add_first(2);
    list.add_first("123".to_string());
    // list.add_first("12往往3".to_string());
    // list.add("sd".to_string());
    // list.update(1, "zhangsan".to_string());

    list.remove_at_index(0);

    println!("{:?}", list);
    // let x = list.remove_at_index(1);
    //
    //
    // let y = list.get(3);
    //
    //
    // println!("{:?}", list);
    // // println!("x = {}", x.unwrap());
    // println!("y = {}", y.unwrap());

    // list.update(2, 100);

    list.into_iter().for_each(|item| {
        println!("{}", item);
    });
}
