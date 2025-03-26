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
    println!("==== date ====");
    let now: DateTime<Utc> = Utc::now();
    let ss: DelayedFormat<StrftimeItems<'_>> = now.format("%d-%m-%Y %H:%M:%S");
    let c_result: CString = CString::new(ss.to_string()).unwrap();
    c_result.into_raw()
}

#[export_name = "developer"]
extern "stdcall" fn developer() -> *const c_char {
    println!("==== developer ====");
    let c_str_song: CString = CString::new("mario ANTUNES").unwrap();
    c_str_song.into_raw()
}

#[export_name = "concatenate"]
extern "stdcall" fn concatenate(one: *const c_char, two: *const c_char) -> *const c_char {
    println!("==== concatenate ====");
    
    // Safely handle potentially null pointers
    if one.is_null() || two.is_null() {
        let error_msg = CString::new("Error: Null pointer provided").unwrap();
        return error_msg.into_raw();
    }
    
    // Get raw bytes from C strings
    let one_bytes = unsafe { CStr::from_ptr(one).to_bytes() };
    let two_bytes = unsafe { CStr::from_ptr(two).to_bytes() };
    
    // Create a new byte vector for the result
    let mut result_bytes = Vec::new();
    result_bytes.extend_from_slice(one_bytes);
    result_bytes.extend_from_slice(b" - ");
    result_bytes.extend_from_slice(two_bytes);
    
    // Create a CString directly from the bytes
    let c_result = match CString::new(result_bytes) {
        Ok(s) => s,
        Err(_) => {
            println!("Error creating CString");
            CString::new("Error: Could not create result string").unwrap()
        }
    };
    
    c_result.into_raw()
}