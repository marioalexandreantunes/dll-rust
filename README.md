# dll-rust

A Windows DLL library written in Rust.

## Overview
This project builds a dynamic library (DLL) for Windows, exposing several functions through a C-compatible FFI interface.

## Building
```sh
# Build the library for the default target (i686-pc-windows-msvc)
cargo build --release
```

## Usage
After building, the generated `rust_dll.dll` can be loaded from other languages (e.g., C, C++, or AutoIt) and the exported functions (`date`, `developer`, `concatenate`, `free_string`) can be called.

## AutoIt Example
The `main.au3` script demonstrates how to use the DLL from AutoIt:
```autoit
; AutoIt x86 - Version with proper memory management
Global $hDLL = DllOpen("target\i686-pc-windows-msvc\release\rust_dll.dll")

; Call developer
Global $aCall_dll = DllCall($hDLL, "ptr", "developer")
If UBound($aCall_dll) <> 0 And $aCall_dll[0] <> 0 Then
    Global $devString = DllStructGetData(DllStructCreate("char[256]", $aCall_dll[0]), 1)
    ConsoleWrite($devString & @CRLF)
    DllCall($hDLL, "none", "free_string", "ptr", $aCall_dll[0])
EndIf

; Call date
$aCall_dll = DllCall($hDLL, "ptr", "date")
If UBound($aCall_dll) <> 0 And $aCall_dll[0] <> 0 Then
    Global $dateString = DllStructGetData(DllStructCreate("char[256]", $aCall_dll[0]), 1)
    ConsoleWrite($dateString & @CRLF)
    DllCall($hDLL, "none", "free_string", "ptr", $aCall_dll[0])
EndIf

; Call concatenate
Global $sUm = "Olá"
Global $sDois = "Mundo"
$aCall_dll = DllCall($hDLL, "ptr", "concatenate", "str", $sUm, "str", $sDois)
If UBound($aCall_dll) <> 0 And $aCall_dll[0] <> 0 Then
    Global $concatString = DllStructGetData(DllStructCreate("char[512]", $aCall_dll[0]), 1)
    ConsoleWrite($concatString & @CRLF)
    DllCall($hDLL, "none", "free_string", "ptr", $aCall_dll[0])
EndIf

DllClose($hDLL)
```

## License
See the `LICENSE.txt` file for licensing information.
