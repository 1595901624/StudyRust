// #[derive(Debug, PartialEq)]
// pub struct LinkedList<T> {
//     // 长度
//     size: usize,
//
//     // 第一个节点
//     first: Option<Box<Node<T>>>,
// }
//
// #[derive(Debug, PartialEq)]
// pub struct Node<T> {
//     // 数据
//     item: T,
//     // 下一个节点
//     next: Option<Box<Node<T>>>,
// }
//
// impl<T> Node<T> {
//     pub fn new(item: T) -> Node<T> {
//         Self {
//             item,
//             next: None,
//         }
//     }
// }
//
// impl<T> LinkedList<T> {
//     pub fn new() -> LinkedList<T> {
//         Self {
//             size: 0,
//             first: None,
//         }
//     }
//
//     pub fn add(&mut self, item: T) {
//         self.add_option(Option::from(item));
//     }
//
//     pub fn add_option(&mut self, item: Option<T>) {
//         if item.is_none() {
//             return;
//         }
//         if self.first.is_none() {
//             self.first = Option::from(Box::new(Node::new(item.unwrap())));
//         } else {
//             let pointer = self.first.as_mut();
//             // while let None = pointer.unwrap().next {
//             //     pointer.unwrap().next = Option::from(Box::new(Node::new(item.unwrap())));
//             // }
//             // self.first
//             // self.first.as_ref().unwrap().borrow_mut().next = Option::from(Rc::new(RefCell::new(Node::new(item.unwrap()))));
//         }
//         self.size += 1;
//     }
//
//     pub fn remove_at_index(&self, index: usize) -> Option<&'static Node<T>> {
//         if index >= self.size {
//             return None;
//         }
//         for i in 0..self.size {
//             if index != i {}
//         }
//         return None;
//     }
// }
