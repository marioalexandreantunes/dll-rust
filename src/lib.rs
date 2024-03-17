use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{DateTime, Utc};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Tips
// https://jakegoulding.com/rust-ffi-omnibus/string_return/
// cargo build --target=i686-pc-windows-msvc -p rust_dll (32bits)
// cargo build --target=x86_64-pc-windows-msvc -p rust_dll (64bits)

#[export_name = "date"]
extern "stdcall" fn date() -> *const c_char {
    println!("date");
    let now: DateTime<Utc> = Utc::now();
    let ss: DelayedFormat<StrftimeItems<'_>> = now.format("%d-%m-%Y %H:%M:%S");
    let c_result: CString = CString::new(ss.to_string()).unwrap();
    c_result.into_raw()
}

#[export_name = "developer"]
extern "stdcall" fn developer() -> *const c_char {
    let c_str_song: CString = CString::new("mario ANTUNES").unwrap();
    c_str_song.into_raw()
}

#[export_name = "concatenate"]
extern "stdcall" fn concatenate(one: *const c_char, two: *const c_char) -> *const c_char {
    let c_str: &CStr = unsafe {
        assert!(!one.is_null());
        CStr::from_ptr(one)
    };
    let one_str: &str = c_str.to_str().unwrap();

    let s_str: &CStr = unsafe {
        assert!(!two.is_null());
        CStr::from_ptr(two)
    };
    let two_str: &str = s_str.to_str().unwrap();

    let result: String = one_str.to_string() + " - " + &*two_str;
    let c_result: CString = CString::new(&*result).unwrap();
    c_result.into_raw()
}