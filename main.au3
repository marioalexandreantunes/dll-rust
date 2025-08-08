; =====================
; AutoIt example for rust_dll.dll (Windows)
; =====================
; This script demonstrates how to:
;   - Load the Rust-built DLL (32-bit path shown below)
;   - Call exported functions that return heap-allocated C strings
;   - Read the returned strings using DllStructCreate
;   - Free the memory on the DLL side using `free_string`
;
; Important
;   - The DLL returns pointers to heap-allocated strings. You must call `free_string` once per pointer.
;   - Ensure the path and CPU architecture match your environment:
;       * For 32-bit DLL:  target\i686-pc-windows-msvc\release\rust_dll.dll (AutoIt must run as 32-bit)
;       * For 64-bit DLL:  target\x86_64-pc-windows-msvc\release\rust_dll.dll (AutoIt must run as 64-bit)
;   - Buffer sizes (char[256], char[512]) must be large enough for your strings; increase if needed.
;   - Encoding: the DLL uses UTF-8 internally. AutoIt may treat char buffers as ANSI. For fully correct UTF-8 handling,
;     consider reading bytes and converting with BinaryToString(..., 4). For simple ASCII/ANSI content, this approach works.
;
; AutoIt x86 - Version with correct memory management
Global $hDLL = DllOpen("target\i686-pc-windows-msvc\release\rust_dll.dll")

; developer function
Global $aCall_dll = DllCall($hDLL, "ptr", "developer")
If UBound($aCall_dll) <> 0 And $aCall_dll[0] <> 0 Then
    ; Read the returned C string (ensure buffer is large enough)
    Global $devString = DllStructGetData(DllStructCreate("char[256]", $aCall_dll[0]), 1)
    ConsoleWrite($devString & @CRLF)
    ; Free memory allocated by the DLL
    DllCall($hDLL, "none", "free_string", "ptr", $aCall_dll[0])
EndIf

; date function
$aCall_dll = DllCall($hDLL, "ptr", "date")
If UBound($aCall_dll) <> 0 And $aCall_dll[0] <> 0 Then
    ; Read the returned C string (ensure buffer is large enough)
    Global $dateString = DllStructGetData(DllStructCreate("char[256]", $aCall_dll[0]), 1)
    ConsoleWrite($dateString & @CRLF)
    ; Free memory allocated by the DLL
    DllCall($hDLL, "none", "free_string", "ptr", $aCall_dll[0])
EndIf

; concatenate function
Global $sUm = "Olá"
Global $sDois = "Mundo"

$aCall_dll = DllCall($hDLL, "ptr", "concatenate", "str", $sUm, "str", $sDois)
If UBound($aCall_dll) <> 0 And $aCall_dll[0] <> 0 Then
    ; Read the returned C string (ensure buffer is large enough)
    Global $concatString = DllStructGetData(DllStructCreate("char[512]", $aCall_dll[0]), 1)
    ConsoleWrite($concatString & @CRLF)
    ; Free memory allocated by the DLL
    DllCall($hDLL, "none", "free_string", "ptr", $aCall_dll[0])
EndIf

DllClose($hDLL)
