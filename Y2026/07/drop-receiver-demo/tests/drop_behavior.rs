use std::cell::RefCell;
use std::rc::Rc;

/// 测试用的日志，便于精确检查析构顺序。
type Log = Rc<RefCell<Vec<&'static str>>>;

struct Field {
    name: &'static str,
    log: Log,
}

impl Drop for Field {
    fn drop(&mut self) {
        self.log.borrow_mut().push(self.name);
    }
}

struct Container {
    log: Log,
    // log 本身不打印，下面两个字段用于观察声明顺序。
    first: Field,
    second: Field,
}

impl Drop for Container {
    fn drop(&mut self) {
        // 自定义 Drop 执行时，两个字段仍然完整可用。
        debug_assert_eq!(self.first.name, "first");
        debug_assert_eq!(self.second.name, "second");
        self.log.borrow_mut().push("container");
    }
}

#[test]
fn custom_drop_runs_before_fields() {
    let log = Rc::new(RefCell::new(Vec::new()));

    {
        let _container = Container {
            log: Rc::clone(&log),
            first: Field {
                name: "first",
                log: Rc::clone(&log),
            },
            second: Field {
                name: "second",
                log: Rc::clone(&log),
            },
        };
    }

    // 自定义 Drop 先执行，随后字段按照声明顺序析构。
    assert_eq!(&*log.borrow(), &["container", "first", "second"]);
}

struct Handle {
    log: Log,
}

impl Drop for Handle {
    fn drop(&mut self) {
        self.log.borrow_mut().push("handle");
    }
}

struct Task {
    log: Log,
    handle: Option<Handle>,
}

impl Drop for Task {
    fn drop(&mut self) {
        self.log.borrow_mut().push("task");

        // take 把字段替换为 None，取出的 Handle 在当前作用域结束时析构。
        let _handle = self.handle.take();
    }
}

#[test]
fn option_take_drops_the_field_exactly_once() {
    let log = Rc::new(RefCell::new(Vec::new()));

    {
        let _task = Task {
            log: Rc::clone(&log),
            handle: Some(Handle {
                log: Rc::clone(&log),
            }),
        };
    }

    assert_eq!(&*log.borrow(), &["task", "handle"]);
}
