#![allow(non_snake_case)]
//! Windows DLL (cdylib) exposing a small C-compatible FFI for string utilities.
//!
//! Overview
//! - date() -> const char* — returns current UTC date/time formatted as "dd-mm-YYYY HH:MM:SS"
//! - developer() -> const char* — returns the developer name
//! - concatenate(const char* s1, const char* s2) -> const char* — returns "s1 - s2"
//! - free_string(char* ptr) — frees a string previously returned by this DLL
//!
//! ABI and platform notes
//! - All exported functions use the Windows-friendly calling convention: `extern "stdcall"`.
//! - The functions are exported with fixed symbol names via `#[export_name = "..."]` so consumers can link by name.
//! - Returned strings are allocated on the Rust heap and must be released by calling `free_string` exactly once.
//! - Strings produced by this DLL are UTF-8 when the source is UTF-8 (date and developer are always UTF-8). The
//!   concatenate function copies raw bytes from the inputs without validating encoding.
//!
//! Safety and ownership
//! - Never free returned pointers with `free`, `LocalFree`, or any other allocator. Only call `free_string`.
//! - Do not call `free_string` more than once for the same pointer.
//! - Do not pass null pointers to functions unless documented. `concatenate` returns an error string if given nulls.
//! - Inputs to `concatenate` must be valid, NUL-terminated C strings; embedded NULs (before the terminator) are not supported.
//!
//! Building
//! - 32-bit:  `cargo build --release --target i686-pc-windows-msvc`
//! - 64-bit:  `cargo build --release --target x86_64-pc-windows-msvc`
//!
//! Example (C, MSVC)
//! ```c
//! // Assume the DLL is named rust_dll.dll and available on PATH or beside the executable.
//! __declspec(dllimport) const char* __stdcall date(void);
//! __declspec(dllimport) const char* __stdcall developer(void);
//! __declspec(dllimport) const char* __stdcall concatenate(const char* s1, const char* s2);
//! __declspec(dllimport) void        __stdcall free_string(char* ptr);
//!
//! void example(void) {
//!     const char* p_dev  = developer();
//!     const char* p_date = date();
//!     const char* p_join = concatenate("Hello", "World");
//!
//!     // Use the strings, then free them with the DLL's allocator
//!     free_string((char*)p_dev);
//!     free_string((char*)p_date);
//!     free_string((char*)p_join);
//! }
//! ```
//!
//! Example (Python ctypes)
//! ```python
//! import ctypes, os, platform
//! dll_path = os.path.join(os.getcwd(), 'target', 'x86_64-pc-windows-msvc', 'release', 'rust_dll.dll')
//! lib = ctypes.WinDLL(dll_path)
//! lib.developer.restype = ctypes.c_void_p
//! lib.date.restype = ctypes.c_void_p
//! lib.concatenate.restype = ctypes.c_void_p
//! lib.concatenate.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
//! lib.free_string.argtypes = [ctypes.c_void_p]
//!
//! def take_and_free(ptr):
//!     s = ctypes.string_at(ptr).decode('utf-8', errors='replace')
//!     lib.free_string(ptr)
//!     return s
//!
//! print(take_and_free(lib.developer()))
//! print(take_and_free(lib.date()))
//! print(take_and_free(lib.concatenate(b"Olá", b"Mundo")))
//! ```
//!
//! Example (AutoIt)
//! ```autoit
//! Global $hDLL = DllOpen("target\\i686-pc-windows-msvc\\release\\rust_dll.dll")
//! Local $aCall = DllCall($hDLL, "ptr", "developer")
//! Local $pDev = $aCall[0]
//! ; read bytes from $pDev as needed, then free with:
//! DllCall($hDLL, "none", "free_string", "ptr", $pDev)
//! DllClose($hDLL)
//! ```

use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{DateTime, Utc};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Tips
// https://jakegoulding.com/rust-ffi-omnibus/string_return/
// cargo build --target=i686-pc-windows-msvc -p rust_dll (32bits)
// cargo build --target=x86_64-pc-windows-msvc -p rust_dll (64bits)

/// Returns the current UTC date and time as a newly allocated C string.
///
/// Format: `dd-mm-YYYY HH:MM:SS` (e.g., `31-12-2025 23:59:59`).
///
/// Ownership
/// - The returned pointer is owned by the caller and must be released with [`free_string`].
/// - Do not attempt to free the pointer with any other allocator.
///
/// Returns
/// - `const char*` pointing to a NUL-terminated UTF-8 string.
///
/// Errors
/// - This function is effectively infallible. In the unlikely event of allocation failure, the
///   process may abort depending on build settings.
#[unsafe(export_name = "date")]
extern "stdcall" fn date() -> *const c_char {
    println!("==== date ====");
    let now: DateTime<Utc> = Utc::now();
    let ss: DelayedFormat<StrftimeItems<'_>> = now.format("%d-%m-%Y %H:%M:%S");
    let c_result: CString = CString::new(ss.to_string()).unwrap();
    c_result.into_raw()
}

/// Returns the developer name as a newly allocated C string (UTF-8).
///
/// Ownership
/// - The returned pointer must be freed with [`free_string`].
///
/// Returns
/// - `const char*` pointing to a NUL-terminated UTF-8 string.
#[unsafe(export_name = "developer")]
extern "stdcall" fn developer() -> *const c_char {
    println!("==== developer ====");
    let c_str_song: CString = CString::new("mario ANTUNES").unwrap();
    c_str_song.into_raw()
}

/// Concatenates two C strings with the separator `" - "` and returns a newly allocated C string.
///
/// Parameters
/// - `one`: pointer to a valid, NUL-terminated C string (bytes are copied as-is)
/// - `two`: pointer to a valid, NUL-terminated C string (bytes are copied as-is)
///
/// Behavior
/// - The bytes are read up to the first NUL in each input and concatenated as `one + " - " + two`.
/// - The function does not validate that the inputs are UTF-8. If non-UTF-8 bytes are passed in, the
///   output will also contain non-UTF-8 bytes.
///
/// Ownership
/// - The returned pointer must be freed with [`free_string`].
///
/// Errors
/// - If either pointer is null, an error message string (allocated) is returned: `"Error: Null pointer provided"`.
///   This string must still be freed by the caller using [`free_string`].
/// - If there is an internal error creating the string, an allocated error string is returned
///   (`"Error: Could not create result string"`), which must also be freed.
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
///
/// Safety
/// - `ptr` must have been returned by one of this library's functions and not have been freed already.
/// - Passing any other pointer, or double-freeing, results in undefined behavior.
///
/// Notes
/// - It is safe to pass a null pointer; the function will do nothing.
#[unsafe(export_name = "free_string")]
extern "stdcall" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            // Reclaim ownership of the allocation and drop it
            let _ = CString::from_raw(ptr);
        }
    }
}
