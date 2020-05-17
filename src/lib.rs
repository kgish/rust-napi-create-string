use nodejs_sys::{napi_create_string_utf8, napi_env, napi_set_named_property, napi_value};
use std::ffi::CString;
//
// Register module
// The N-API documentation recommends `NAPI_MODULE_INIT()` macro for module registration which
// compiles to the `napi_register_module_v1` function.
//
#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
    env: napi_env,
    exports: napi_value,
) -> nodejs_sys::napi_value {
    let name: CString = CString::new("greeting").expect("CString::new failed");

    let value: napi_value = create_string(env, "Hello, world!");

    // Set named property
    // https://docs.rs/nodejs-sys/0.3.0/nodejs_sys/fn.napi_set_named_property.html
    //
    // pub unsafe extern "C" fn napi_set_named_property(
    //     env: napi_env,
    //     object: napi_value,
    //     utf8name: *const c_char,
    //     value: napi_value
    // ) -> napi_status
    //
    napi_set_named_property(env, exports, name.as_ptr(), value);

    exports
}

unsafe fn create_string(env: napi_env, t: &str) -> napi_value {
    let mut result: napi_value = std::mem::zeroed();
    let s = CString::new(t).expect("CString::new failed");

    napi_create_string_utf8(env, s.as_ptr(), s.as_bytes().len(), &mut result);

    result
}
