use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{DateTime, Utc};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Tips
// https://jakegoulding.com/rust-ffi-omnibus/string_return/
// cargo build --target=i686-pc-windows-msvc -p rust_dll (32bits)
// cargo build --target=x86_64-pc-windows-msvc -p rust_dll (64bits)

/// Returns the current date and time as a C string in the format "dd-mm-YYYY HH:MM:SS".
/// The caller is responsible for freeing the returned string using `free_string`.
#[unsafe(export_name = "date")]
extern "stdcall" fn date() -> *const c_char {
    println!("==== date ====");
    let now: DateTime<Utc> = Utc::now();
    let ss: DelayedFormat<StrftimeItems<'_>> = now.format("%d-%m-%Y %H:%M:%S");
    let c_result: CString = CString::new(ss.to_string()).unwrap();
    c_result.into_raw()
}

/// Returns the developer name as a C string.
/// The caller must free the returned string using `free_string`.
#[unsafe(export_name = "developer")]
extern "stdcall" fn developer() -> *const c_char {
    println!("==== developer ====");
    let c_str_song: CString = CString::new("mario ANTUNES").unwrap();
    c_str_song.into_raw()
}

/// Concatenates two C strings with a separator `" - "` and returns a new C string.
/// Returns an error C string if either input pointer is null or if string creation fails.
/// The caller must free the returned string using `free_string`.
#[unsafe(export_name = "concatenate")]
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

/// Frees a C string previously allocated by this library.
/// Safety: The pointer must have been returned by one of the library's functions and not freed already.
#[unsafe(export_name = "free_string")]
extern "stdcall" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            // Reconstrói o CString e deixa que seja dropado automaticamente
            let _ = CString::from_raw(ptr);
        }
    }
}
