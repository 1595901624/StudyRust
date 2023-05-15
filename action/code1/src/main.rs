fn main() {
    let value = vec![5, 12, 23, 45, 65, 66, 87, 98];
    let result = binary_search(value, 12);
    println!("查找结果: {}", result);
}

/// 二分查找
/// `element` 待查找的元素
/// `value` 待查找的数组
fn binary_search(value: Vec<i32>, element: i32) -> i32 {
    // 初始化 两个端点的索引值
    let mut start = 0;
    let mut end = value.len() - 1;

    // 满足 左侧端点的索引值 小于等于 右侧端点的索引值 才会循环
    while start <= end {
        // 计算搜索区间的中间位置
        let mid = start + (end - start) / 2;
        // 如果待查找元素与中间元素相等，则查找成功，返回 mid
        if value[mid] == element {
            // mid 默认是 usize 类型，需要使用 as 转换为 i32 类型
            return mid as i32;
        } else if value[mid] > element {
            // 如果待查找元素比中间元素小，则更新 end = mid - 1，
            end = mid - 1;
        } else {
            // 如果待查找元素比中间元素大，则更新 start = mid + 1，
            start = mid + 1;
        }
    }
    // 如果 start > end，则表明数组中不存在待查找元素，则查找失败，返回默认值 -1
    return -1;
}

#[test]
fn test() {
    // let array = vec![0, 1, 2, 4, 2, 1];
    let array = vec![1, 4, 7, 9, 8, 6, 4, 2];
    let target = 2;
    let result = find_in_mountain_array(target, array);
    assert_eq!(7, result);
}

/// 山脉数组中查找目标值
pub fn find_in_mountain_array(target: i32, array: Vec<i32>) -> i32 {
    // 首先通过二分法找山顶
    let mut start = 0;
    let mut end = array.len() - 1;
    // 按照山脉数组的要求，最后一个元素肯定不是峰值，所以排除掉
    while start < end {
        let mid = start + (end - start) / 2;
        if array[mid] < array[mid + 1] {
            start = mid + 1;
        } else {
            end = mid;
        }
    }
    // 封顶值是 start
    let peak = start;
    let result = bs(target, &array, 0, peak, false);
    if result != -1 {
        return result;
    }
    return bs(target, &array, peak + 1, array.len() - 1, true);
}

/// 二分查找
fn bs(target: i32, array: &Vec<i32>, mut start: usize, mut end: usize, rev: bool) -> i32 {
    // 获取新的 target 值
    // 为了实现代码复用，我们仅写一个二分查找的方法
    // 对于右侧的值，如果我们将当前的值 * -1，则将变成升序排列
    // 首先将 target 的值 * -1
    let new_target;
    if rev {
        new_target = target * -1;
    } else {
        new_target = target;
    }

    // 满足 左侧端点的索引值 小于等于 右侧端点的索引值 才会循环
    while start <= end {
        // 计算搜索区间的中间位置
        let mid = start + (end - start) / 2;
        // 我们并不需要将所有的值都 * -1，只需要将每次查找到的中间元素 * -1 与 新的 target 值比较
        let current_element = array[mid] * if rev { -1 } else { 1 };
        // 如果待查找元素与中间元素相等，则查找成功，返回 mid
        if current_element == new_target {
            // mid 默认是 usize 类型，需要使用 as 转换为 i32 类型
            return mid as i32;
        } else if current_element > new_target {
            // 如果待查找元素比中间元素小，则更新 end = mid - 1，
            end = mid - 1;
        } else {
            // 如果待查找元素比中间元素大，则更新 start = mid + 1，
            start = mid + 1;
        }
    }
    // 如果 start > end，则表明数组中不存在待查找元素，则查找失败，返回默认值 -1
    return -1;
}