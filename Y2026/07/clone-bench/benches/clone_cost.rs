use std::hint::black_box;
use std::sync::Arc;

use criterion::{Criterion, criterion_group, criterion_main};

/// 对借用的数据做一次轻量读取，让借用组产生可观察的结果。
fn read_slice(data: &[u64]) -> u64 {
    data.len() as u64 + data.first().copied().unwrap_or_default()
}

fn clone_cost(c: &mut Criterion) {
    // 1 KiB 的字符串，用于观察 String 深复制时的内存分配和数据复制。
    let text = "R".repeat(1024);
    // u64 占 8 字节，100_000 个元素的数据量约为 800 KiB。
    let numbers = vec![7_u64; 100_000];
    // 在计时开始前创建 Arc，避免把初次分配的成本算进 Arc::clone。
    let shared_numbers = Arc::new(numbers.clone());

    c.bench_function("clone String 1 KiB", |b| {
        // String::clone 会重新分配堆内存，并复制字符串内容。
        b.iter(|| black_box(text.clone()));
    });

    c.bench_function("clone Vec 800 KiB", |b| {
        // Vec::clone 会重新分配缓冲区，并复制全部元素。
        b.iter(|| black_box(numbers.clone()));
    });

    c.bench_function("clone Arc<Vec>", |b| {
        // Arc::clone 共享原来的 Vec，只增加原子引用计数。
        b.iter(|| black_box(Arc::clone(&shared_numbers)));
    });

    c.bench_function("borrow Vec", |b| {
        // 借用不会创建新的所有者，black_box 用于避免编译器消除测量代码。
        b.iter(|| black_box(read_slice(black_box(numbers.as_slice()))));
    });
}

criterion_group!(benches, clone_cost);
criterion_main!(benches);
