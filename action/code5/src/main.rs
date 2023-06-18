use crate::linked_list::LinkedList;

mod linked_list;
mod test;

fn main() {
    let mut list = LinkedList::new();

    println!("----向尾部添加元素----");
    list.add("ZhangSan".to_string());
    list.add("LiSi".to_string());
    list.add("WangWu".to_string());
    println!("{:?}", list);
    println!();

    println!("----向头部添加元素----");
    list.add_first("Tom".to_string());
    list.add_first("Jonh".to_string());
    println!("{:?}", list);
    println!();

    println!("----移除第0个元素----");
    list.remove_at_index(0);
    println!("{:?}", list);
    println!();

    println!("----移除第最后1个元素----");
    list.remove_at_index(list.size() - 1);
    println!("{:?}", list);
    println!();


    println!("----更新第0个元素的值为XiaoMing----");
    list.update(0, "XiaoMing".to_string());
    println!("{:?}", list);
    println!();

    println!("----获取第0个元素的值----");
    let ele = list.get(0);
    println!("{:?}", ele);
    println!();

    println!("----迭代器----");
    list.into_iter().for_each(|item| {
        println!("{}", item);
    });
}
