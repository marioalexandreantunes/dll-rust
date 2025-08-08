# dll-rust

A small Windows DLL written in Rust that exposes a C-compatible FFI.

The library returns heap-allocated C strings (UTF-8) that must be freed by the caller using the provided `free_string` function.

- Exported functions
  - `const char* date(void)` – current date/time in the format `dd-mm-YYYY HH:MM:SS`
  - `const char* developer(void)` – developer name
  - `const char* concatenate(const char* s1, const char* s2)` – concatenates `s1` + " - " + `s2`
  - `void free_string(char* ptr)` – frees a string previously returned by the DLL

Notes
- Functions are exported with a C ABI and a Windows-friendly calling convention (extern "system").
- All returned strings are owned by the DLL. You must call `free_string` once you are done with them.
- Returned strings are UTF-8. If your consumer expects ANSI or UTF-16, convert accordingly.

## Requirements
- Rust (stable) with MSVC toolchain on Windows
- For 64-bit builds: target `x86_64-pc-windows-msvc`
- For 32-bit builds: target `i686-pc-windows-msvc` (useful for AutoIt x86)

## Build

- Default (as configured by your toolchain):
```sh
cargo build --release
```

- Explicit 64-bit build:
```sh
rustup target add x86_64-pc-windows-msvc
cargo build --release --target=x86_64-pc-windows-msvc
```
Resulting DLL:
```
target/x86_64-pc-windows-msvc/release/rust_dll.dll
```

- Explicit 32-bit build (often required for AutoIt x86):
```sh
rustup target add i686-pc-windows-msvc
cargo build --release --target=i686-pc-windows-msvc
```
Resulting DLL:
```
target/i686-pc-windows-msvc/release/rust_dll.dll
```

## Exported API (C header style)

```c
// Returned strings are UTF-8 and must be freed with free_string
const char* date(void);
const char* developer(void);
const char* concatenate(const char* s1, const char* s2);
void        free_string(char* ptr);
```

Behavior and error handling
- `concatenate` will return an error message as a C string if either input pointer is null or if allocation fails. You must still call `free_string` on that returned pointer.
- `free_string` is safe to call with `NULL` (no-op), but only pass pointers that were returned by this DLL.

## Using from AutoIt (x86 example)

Below is a minimal example that loads the 32-bit DLL, calls the functions, prints results, and frees memory. Adjust the DLL path to your build output and make sure the AutoIt process bitness matches the DLL architecture.

```autoit
; AutoIt (x86) example
Global $dllPath = @ScriptDir & "\\target\\i686-pc-windows-msvc\\release\\rust_dll.dll"
Global $hDLL = DllOpen($dllPath)
If @error Or $hDLL = -1 Then
    ConsoleWrite("Failed to load DLL: " & $dllPath & "\n")
    Exit 1
EndIf

; Helper to read a null-terminated C string (up to a max length)
Func _PtrToCString($p, $max = 1024)
    If $p = 0 Then Return ""
    Local $ds = DllStructCreate("char[" & $max & "]", $p)
    Local $s = DllStructGetData($ds, 1)
    ; Truncate at the first null (DllStructGetData may already stop at null)
    Local $iNull = StringInStr($s, Chr(0))
    If $iNull > 0 Then $s = StringLeft($s, $iNull - 1)
    Return $s
EndFunc

; developer()
Local $ret = DllCall($hDLL, "ptr", "developer")
If @error Or Not IsArray($ret) Or $ret[0] = 0 Then
    ConsoleWrite("developer() failed\n")
Else
    Local $pStr = $ret[0]
    Local $s = _PtrToCString($pStr, 256)
    ConsoleWrite("developer: " & $s & "\n")
    DllCall($hDLL, "none", "free_string", "ptr", $pStr)
EndIf

; date()
$ret = DllCall($hDLL, "ptr", "date")
If @error Or Not IsArray($ret) Or $ret[0] = 0 Then
    ConsoleWrite("date() failed\n")
Else
    Local $pStr = $ret[0]
    Local $s = _PtrToCString($pStr, 256)
    ConsoleWrite("date: " & $s & "\n")
    DllCall($hDLL, "none", "free_string", "ptr", $pStr)
EndIf

; concatenate("Olá", "Mundo")
Local $sOne = "Olá"
Local $sTwo = "Mundo"
$ret = DllCall($hDLL, "ptr", "concatenate", "str", $sOne, "str", $sTwo)
If @error Or Not IsArray($ret) Or $ret[0] = 0 Then
    ConsoleWrite("concatenate() failed\n")
Else
    Local $pStr = $ret[0]
    Local $s = _PtrToCString($pStr, 512)
    ConsoleWrite("concatenate: " & $s & "\n")
    DllCall($hDLL, "none", "free_string", "ptr", $pStr)
EndIf

DllClose($hDLL)
```

Tips for AutoIt
- Match architecture: AutoIt x86 must load a 32-bit DLL; AutoIt x64 must load a 64-bit DLL.
- The example treats returned data as a C `char*`. If you see garbled characters for non-ASCII text, convert from UTF-8 according to your needs.

## Using from Python

Below is a ctypes example that loads the DLL, calls the exported functions, decodes the returned UTF-8 strings, and frees the underlying memory via `free_string`. Use WinDLL to respect the Windows calling convention. Ensure the DLL architecture matches your Python interpreter (32-bit vs 64-bit).

```python
import ctypes
import os
import platform

# Choose the DLL path according to your build/architecture
root = os.path.dirname(__file__)
if platform.architecture()[0] == '64bit':
    dll_path = os.path.join(root, 'target', 'x86_64-pc-windows-msvc', 'release', 'rust_dll.dll')
else:
    dll_path = os.path.join(root, 'target', 'i686-pc-windows-msvc', 'release', 'rust_dll.dll')

# Load with WinDLL to match stdcall on 32-bit and default on 64-bit
lib = ctypes.WinDLL(dll_path)

# Prototypes: return pointers (void*) so we can free them later
lib.developer.restype = ctypes.c_void_p
lib.developer.argtypes = []

lib.date.restype = ctypes.c_void_p
lib.date.argtypes = []

lib.concatenate.restype = ctypes.c_void_p
lib.concatenate.argtypes = [ctypes.c_char_p, ctypes.c_char_p]

lib.free_string.restype = None
lib.free_string.argtypes = [ctypes.c_void_p]


def take_string_and_free(ptr: int) -> str:
    """Convert a returned char* (UTF-8) to Python str and free it via free_string."""
    if not ptr:
        return ''
    # Read bytes from pointer
    raw = ctypes.cast(ptr, ctypes.c_char_p).value or b''
    # Free the original allocation
    lib.free_string(ctypes.c_void_p(ptr))
    # Decode UTF-8 with replacement for safety
    return raw.decode('utf-8', errors='replace')


# developer()
p_dev = lib.developer()
print('developer:', take_string_and_free(p_dev))

# date()
p_date = lib.date()
print('date:', take_string_and_free(p_date))

# concatenate("Olá", "Mundo") — pass UTF-8 encoded bytes
p_concat = lib.concatenate('Olá'.encode('utf-8'), 'Mundo'.encode('utf-8'))
print('concatenate:', take_string_and_free(p_concat))
```

Notes for Python
- Always set restype to `c_void_p` for functions returning allocated strings so you can free the original pointer with `free_string`.
- Decode using UTF-8. If your environment expects a different encoding, convert accordingly.
- Ensure the DLL's bitness matches your Python interpreter (use `python -c "import platform; print(platform.architecture())"`).
