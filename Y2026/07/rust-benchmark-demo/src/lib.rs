/// 使用稳定排序将数组按升序排列
/// 稳定排序会保留相等元素的相对顺序，适合排序结果还依赖原始顺序的业务对象
pub fn stable_sort(values: &mut [u32]) {
    values.sort();
}

/// 使用不稳定排序将数组按升序排列
/// 不稳定排序并不会保证相等元素的相对顺序，可能会有更低的额外的性能开销
pub fn unstable_sort(values: &mut [u32]) {
    values.sort_unstable();
}

#[cfg(test)]
mod tests {
    use super::{stable_sort, unstable_sort};

    #[test]
    fn both_sorts_produce_ascending_values() {
        // 两种实现使用同一份输入，确保这里只验证排序结果，不引入额外差异
        let input = [8, 3, 9, 1, 5, 2, 7, 4, 6];
        let expected = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        let mut stable = input;
        stable_sort(&mut stable);
        assert_eq!(stable, expected);

        let mut unstable = input;
        unstable_sort(&mut unstable);
        assert_eq!(unstable, expected);
    }
}
