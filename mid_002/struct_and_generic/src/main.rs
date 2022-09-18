struct Person {
    // 名字
    name: String,
    // 年龄
    age: u8,
}

impl Person {
    // 方法
    fn get_age(&self) -> u8 {
        return self.age;
    }

    fn set_age(&mut self, age: u8) {
        self.age = age;
    }

    fn get_name(&self) -> &str {
        return self.name.as_str();
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    // 关联函数
    fn to_string(person: Person) -> String {
        return format!("Person {{ name: {}, age: {} }}", person.get_name(), person.get_age());
    }
}

/// 定义一个队列
struct Queue<T> {
    // 通过向量存储数据
    data: Vec<T>,
    // 队列的长度
    size: u32,
}

///  定义队列的方法
impl<T> Queue<T> {
    /// 创建一个队列
    /// 这是一个关联函数
    pub fn new() -> Queue<T> {
        Queue {
            data: vec![],
            size: 0,
        }
    }

    /// 向队列中插入一个值
    /// 这是一个方法
    pub fn offer(&mut self, value: T) {
        self.data.push(value);
        self.size += 1;
    }

    /// 弹出队列的最顶部的元素
    /// 这是一个方法
    pub fn poll(&mut self) {
        if self.size > 0 {
            self.data.remove(0);
            self.size -= 1;
        }
    }
}

fn main() {
    let mut person = Person {
        name: String::from("test"),
        age: 8,
    };

    println!("修改前：name: {}, age: {}", person.get_name(), person.get_age());

    person.set_name("张三");
    person.set_age(18);

    println!("修改后：name: {}, age: {}", person.get_name(), person.get_age());

    // 通过关联函数输出
    println!("{}", Person::to_string(person));

    // 输出结果：
    // 修改前：name: test, age: 8
    // 修改后：name: 张三, age: 18
    // Person { name: 张三, age: 18 }

    // 下面是泛型结构体
    let mut queue: Queue<i32> = Queue::new();
    queue.offer(4);
    queue.offer(7);
    queue.offer(10);
    println!("队列的长度：{}", queue.size);
    queue.poll();
    queue.poll();
    queue.poll();
    queue.offer(10);
    println!("队列的长度：{}", queue.size);

    // 输出结果：
    // 队列的长度：3
    // 队列的长度：1
}
