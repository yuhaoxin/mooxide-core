//! mooxide 的 C ABI 绑定。头文件由 cbindgen 从本文件导出的
//! `extern "C"` 函数生成（`cargo xtask ffi-header`），C 消费方
//! 只依赖 `include/mooxide/mooxide.h`，不接触 Rust 符号。
//!
//! # 约定
//! - 所有返回字符串的函数返回裸指针，释放必须调用对应的 `*_free`。
//! - 指针一旦传入 `*_free` 即失效，重复释放是未定义行为。

use std::ffi::{CStr, CString, c_char};

/// 释放 [`mooxide_version`] 返回的字符串。
///
/// # Safety
/// `ptr` 必须是 [`mooxide_version`] 的返回值且未被释放过。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mooxide_version_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        // SAFETY: ptr 非空且由 CString::into_raw 产生（见 mooxide_version）。
        drop(unsafe { CString::from_raw(ptr) });
    }
}

/// 返回库版本号字符串，调用方用 [`mooxide_version_free`] 释放。
#[unsafe(no_mangle)]
pub extern "C" fn mooxide_version() -> *mut c_char {
    let v = CString::new(env!("CARGO_PKG_VERSION")).unwrap_or_default();
    v.into_raw()
}

/// 校验 C 侧能正常往返一个字符串，作为 FFI 链路的冒烟占位。
///
/// 返回 0 表示 `input` 非空，1 表示为空或非法 UTF-8。
///
/// # Safety
/// `input` 必须指向以 NUL 结尾的合法 C 字符串。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mooxide_smoke(input: *const c_char) -> i32 {
    if input.is_null() {
        return 1;
    }
    // SAFETY: input 已判空且指向 NUL 结尾的 C 字符串（函数 Safety 契约）。
    match unsafe { CStr::from_ptr(input) }.to_bytes() {
        [] => 1,
        [..] => 0,
    }
}
