use std::ffi::{c_char, c_int, c_ulong, CString, c_void};

type Value = usize;

#[link(name = "ruby")]
extern "C" {
    pub fn rb_define_module(name: *const c_char) -> Value;

    pub fn rb_define_module_function(
        module: Value,
        name: *const c_char,
        func: *const c_void,
        argc: c_int
    );

    pub fn rb_num2ulong(num: Value) -> c_ulong;

    pub fn rb_uint2big(l: c_ulong) -> Value;
}

pub extern "C" fn add(_: Value, a: Value, b: Value) -> Value {
    unsafe {
        let longa = rb_num2ulong(a);
        let longb = rb_num2ulong(b);
        let result = multi_bind::add(longa.into(), longb.into());
        rb_uint2big(result)
    }
}

pub extern "C" fn unwrap_future(_: Value) -> Value {
    let result = tokio::runtime::Runtime::new().unwrap().block_on(async {
        multi_bind::do_async().await
    });
    unsafe {
        rb_uint2big(result)
    }
}

#[export_name="Init_multi_bind_rb"]
pub extern "C" fn init_muti_bind_rb() {
    let module : Value;
    unsafe {
        module = rb_define_module(CString::new("MultiBind").unwrap().as_ptr().into());
        rb_define_module_function(module, CString::new("add").unwrap().as_ptr().into(), add as *const c_void, 2);
        rb_define_module_function(module, CString::new("unwrap_future").unwrap().as_ptr().into(), unwrap_future as *const c_void, 0);
    }
}
