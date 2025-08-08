; Autoit x86 - Versão com gestão correta de memória
Global $hDLL = DllOpen("target\i686-pc-windows-msvc\release\rust_dll.dll")

; Função developer
Global $aCall_dll = DllCall($hDLL, "ptr", "developer")
If UBound($aCall_dll) <> 0 And $aCall_dll[0] <> 0 Then
    Global $devString = DllStructGetData(DllStructCreate("char[256]", $aCall_dll[0]), 1)
    ConsoleWrite($devString & @CRLF)
    ; Liberar memória
    DllCall($hDLL, "none", "free_string", "ptr", $aCall_dll[0])
EndIf

; Função date
$aCall_dll = DllCall($hDLL, "ptr", "date")
If UBound($aCall_dll) <> 0 And $aCall_dll[0] <> 0 Then
    Global $dateString = DllStructGetData(DllStructCreate("char[256]", $aCall_dll[0]), 1)
    ConsoleWrite($dateString & @CRLF)
    ; Liberar memória
    DllCall($hDLL, "none", "free_string", "ptr", $aCall_dll[0])
EndIf

; Função concatenate
Global $sUm = "Olá"
Global $sDois = "Mundo"

$aCall_dll = DllCall($hDLL, "ptr", "concatenate", "str", $sUm, "str", $sDois)
If UBound($aCall_dll) <> 0 And $aCall_dll[0] <> 0 Then
    Global $concatString = DllStructGetData(DllStructCreate("char[512]", $aCall_dll[0]), 1)
    ConsoleWrite($concatString & @CRLF)
    ; Liberar memória
    DllCall($hDLL, "none", "free_string", "ptr", $aCall_dll[0])
EndIf

DllClose($hDLL)
