@echo off
rem Restart the dev app with CDP enabled (port 9223) — kill leftovers, set flag, launch
cd /d "%~dp0"
taskkill /f /im data-monster.exe >nul 2>&1
taskkill /f /im cargo.exe >nul 2>&1
taskkill /f /im node.exe >nul 2>&1
taskkill /f /im msedgewebview2.exe >nul 2>&1
set WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9223
npm run dev
