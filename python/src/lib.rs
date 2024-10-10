use pyo3::prelude::*;

#[pymodule]
mod multi_bind_py {

    use super::*;

    #[pyfunction]
    fn add(a: u64, b: u64) -> u64 {
        multi_bind::add(a, b)
    }

    #[pyfunction]
    fn unwrap_future() -> u64 {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            multi_bind::do_async().await
        })
    }
}
