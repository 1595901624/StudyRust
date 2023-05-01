use pyo3::prelude::*;

/// 排序
#[pyfunction]
fn sort(mut vec: Vec<isize>) -> PyResult<Vec<isize>> {
    vec.sort();
    Ok(vec)
}

/// 一个用Rust实现的Python模块。
///
/// 这个函数的名字必须与`Cargo.toml`中的`lib.name`匹配
#[pymodule]
fn python_rust2(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(sort, module)?)?;
    Ok(())
}