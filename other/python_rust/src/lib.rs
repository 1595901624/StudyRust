use pyo3::prelude::*;

/// 求两个数的和
#[pyfunction]
fn sum(a: isize, b: isize) -> PyResult<isize> {
    Ok(a + b)
}

#[pyfunction]
fn multiple(a: isize, b: isize) -> PyResult<isize> {
    Ok(a * b)
}

/// 一个用Rust实现的Python模块。
///
/// 这个函数的名字必须与`Cargo.toml`中的`lib.name`匹配
#[pymodule]
fn python_rust(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(sum, module)?)?;
    module.add_function(wrap_pyfunction!(multiple, module)?)?;
    Ok(())
}