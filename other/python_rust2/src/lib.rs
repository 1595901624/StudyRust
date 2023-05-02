use pyo3::prelude::*;

/// 排序
#[pyfunction]
fn sort(mut vec: Vec<isize>) -> PyResult<Vec<isize>> {
    vec.sort();
    Ok(vec)
}

#[pyclass]
struct Dog {
    #[pyo3(get, set)]
    pub id: isize,
    #[pyo3(get, set)]
    pub name: String,
}

#[pymethods]
impl Dog {
    #[new]
    fn new(id: isize, name: String) -> Self {
        Self {
            id,
            name,
        }
    }

    fn run(&self) {
        println!("{} is running!", self.name);
    }
}

/// 一个用Rust实现的Python模块。
///
/// 这个函数的名字必须与`Cargo.toml`中的`lib.name`匹配
#[pymodule]
fn python_rust2(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(sort, module)?)?;
    module.add_class::<Dog>()?;
    Ok(())
}