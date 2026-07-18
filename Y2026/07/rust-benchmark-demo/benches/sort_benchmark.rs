use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use rust_benchmark_demo::{stable_sort, unstable_sort};
use std::hint::black_box;

/// 生成固定的随机输入，避免每次 Benchmark 的数据分布发生变化
fn build_data(size: usize) -> Vec<u32> {
    let mut seed = 0x1234_5678_u32;

    (0..size)
        .map(|_| {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            seed
        })
        .collect()
}

/// 比较稳定排序和不稳定排序在相同输入规模下的性能
fn benchmark_sorts(c: &mut Criterion) {
    // Benchmark 的输入规模，可根据需要调整
    // 这里选择 10000 个元素，可尝试不同的规模，观察排序算法的性能变化
    const SIZE: usize = 10_000;
    // 两个 benchmark 共享同一份未排序输入，保证比较条件一致
    let input = build_data(SIZE);

    // 创建一个 Benchmark Group，便于对同一类操作进行分组和比较
    let mut group = c.benchmark_group("sort_u32");

    // 将每次操作处理的元素数告诉 Criterion，输出中会额外显示吞吐量
    group.throughput(Throughput::Elements(SIZE as u64));

    group.bench_function("stable", |b| {
        b.iter_batched_ref(
            // 排序会原地修改数据, 我们把数据复制放在 setup 中，不计入排序函数的耗时
            || input.clone(),
            |values| {
                // black_box 避免编译器因结果未被显式使用，从而消除排序操作的优化
                stable_sort(black_box(&mut *values));
                // 让排序后的结果也成为可观察值，避免整个计算被过度优化
                black_box(&*values);
            },
            // 每轮都从同一份乱序输入开始，避免后续的循环轮次测到“已排序数据”
            BatchSize::SmallInput,
        );
    });

    group.bench_function("unstable", |b| {
        b.iter_batched_ref(
            // 与 stable benchmark 使用完全相同的输入生成方式和计时边界
            || input.clone(),
            |values| {
                // 两个排序实现都需要让输入和输出保持可观察，避免测量被优化掉
                unstable_sort(black_box(&mut *values));
                black_box(&*values);
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, benchmark_sorts);
criterion_main!(benches);
