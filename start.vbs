Dim scriptFullPath
scriptFullPath = WScript.ScriptFullName

Dim scriptDir
scriptDir = Left(scriptFullPath, InStrRev(scriptFullPath, "\"))

Dim batFileName
batFileName = ".data\settings.bat"

Dim batFilePath
batFilePath = scriptDir & batFileName

Dim objShell
Set objShell = CreateObject("Shell.Application")

objShell.ShellExecute "cmd.exe", "/c """ & batFilePath & """", "", "", 0
