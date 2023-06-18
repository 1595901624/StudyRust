use std::cell::{RefCell};
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub struct LinkedList<T> where T: PartialEq + Debug + Clone {
    // 长度
    size: usize,

    // 第一个节点
    first: Option<Rc<RefCell<Node<T>>>>,

    // 最后一个节点
    last: Option<Rc<RefCell<Node<T>>>>,
}

// 节点
#[derive(Debug, PartialEq, Clone)]
pub struct Node<T> where T: PartialEq + Debug + Clone {
    // 数据
    item: T,
    // 下一个节点
    next: Option<Rc<RefCell<Node<T>>>>,
}

/// 实现节点
impl<T> Node<T> where T: PartialEq + Debug + Clone {
    pub fn new(item: T) -> Node<T> {
        Self {
            item,
            next: None,
        }
    }

    pub fn new_with_node(item: T, next: Option<Rc<RefCell<Node<T>>>>) -> Node<T> {
        Self {
            item,
            next,
        }
    }
}

impl<T> LinkedList<T> where T: PartialEq + Debug + Clone {
    pub fn new() -> LinkedList<T> {
        Self {
            size: 0,
            first: None,
            last: None,
        }
    }

    /// 添加节点
    pub fn add(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        match self.last.take() {
            Some(l) => l.borrow_mut().next = Some(new_node.clone()),
            None => self.first = Some(new_node.clone()),
        };
        self.last = Some(new_node);
        self.size += 1;
    }

    /// 移除节点
    pub fn remove_at_index(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }
        // 当前节点
        let mut pointer = self.first.clone();
        // 前驱节点
        let mut previous = None;

        // 找到要删除的节点
        for _ in 0..index {
            // 保存前驱节点
            previous = pointer.clone();
            pointer = match pointer {
                Some(node) => node.borrow().next.clone(),
                None => return None,
            };
        }

        // 要删除的节点
        let deleted_node = pointer.unwrap();

        // 更新最后一个节点的信息
        if self.last.as_ref() == Some(&deleted_node) {
            self.last = previous.clone();
        }

        // 更新链表的 first 和 last
        if previous.is_none() {
            self.first = deleted_node.borrow().next.clone();
        } else {
            previous.unwrap().borrow_mut().next = deleted_node.borrow().next.clone();
        }

        self.size -= 1;
        // 返回被删除的节点
        return Some(Rc::try_unwrap(deleted_node).unwrap().into_inner().item);
    }

    /// 获取某节点的值
    pub fn get(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }
        let mut pointer = self.first.clone();
        for _ in 0..index {
            pointer = match pointer {
                Some(node) => node.borrow().next.clone(),
                None => return None,
            };
        }
        let node = pointer.unwrap();
        return Some(node.borrow().item.clone());
    }

    /// 更新某节点的值
    pub fn update(&mut self, index: usize, data: T) -> bool {
        if index >= self.size {
            return false;
        }
        let mut pointer = self.first.clone();

        for _ in 0..index {
            pointer = match pointer {
                Some(node) => node.borrow().next.clone(),
                None => return false,
            };
        }

        pointer.unwrap().borrow_mut().item = data;
        return true;
    }

    /// 添加头节点
    pub fn add_first(&mut self, data: T) {
        if self.size == 0 {
            let node = Node::new(data);
            self.first = Some(Rc::new(RefCell::new(node.clone())));
            self.last = self.first.clone();
        } else {
            let temp = self.first.take();
            let node = Node::new_with_node(data, temp);
            self.first = Some(Rc::new(RefCell::new(node)));
        }

        self.size += 1;
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

// ******************************迭代器 START***********************************

#[derive(Debug)]
pub struct NodeIter<T> where T: PartialEq + Debug + Clone {
    current: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Iterator for NodeIter<T>
    where T: PartialEq + Debug + Clone {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.current.take() {
            Some(node) => {
                let node_ref = node.borrow();
                self.current = node_ref.next.clone();
                Some(node_ref.item.clone())
            }
            None => None,
        };
        result
    }
}

impl<T> IntoIterator for LinkedList<T>
    where T: PartialEq + Debug + Clone {
    type Item = T;
    type IntoIter = NodeIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIter {
            current: self.first.clone(),
        }
    }
}

// ******************************迭代器 E N D***********************************



