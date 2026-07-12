#![feature(unboxed_closures)]

// 异步函数 f 接受一个闭包，该闭包对于所有生命周期 'a，
// 参数为 &'a u8，并返回一个 Future，其输出类型为 ()。
async fn f<Fut>(_: impl for<'a> Fn(&'a u8) -> Fut)
where
    Fut: Future<Output = ()>,
{
    todo!()
}

// 异步函数 f2 接受一个满足 AsyncFn(&'a u8) 特性的闭包（这里假设 AsyncFn 是某个在上下文中定义或者引入的 trait）
async fn f2(_: impl for<'a> AsyncFn(&'a u8)) {
    todo!() // 占位实现
}

fn main() {}
