use std::ffi::{c_ulong};

#[export_name="add"]
pub extern "C" fn add(a: c_ulong, b: c_ulong) -> c_ulong {
    multi_bind::add(a.into(), b.into())
}

#[export_name="unwrap_future"]
pub extern "C" fn unwrap_future() -> c_ulong {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        multi_bind::do_async().await
    })
}
