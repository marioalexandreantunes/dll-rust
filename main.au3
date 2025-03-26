; Autoit x86
Local $hDLL = DllOpen("target\i686-pc-windows-msvc\debug\rust_dll.dll")

Local $aCall_dll = DllCall($hDLL, "STR", "developer")
If UBound($aCall_dll) <> 0 Then
	ConsoleWrite($aCall_dll[0] & @CRLF)
EndIf

$aCall_dll = DllCall($hDLL, "STR", "date")
If UBound($aCall_dll) <> 0 Then
	ConsoleWrite($aCall_dll[0] & @CRLF)
EndIf

Local $sUm = "Olá"
Local $sDois = "Mundo"

$aCall_dll = DllCall($hDLL, "STR", "concatenate", "STR", $sUm, "STR", $sDois)
If UBound($aCall_dll) <> 0 Then
	ConsoleWrite($aCall_dll[0] & @CRLF)
EndIf

DllClose($hDLL)

