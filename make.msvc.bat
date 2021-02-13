git submodule init
git submodule update --recursive
faust -os -cn Effects -o build/Effects.h audio/Effects.dsp
copy desktop\webview\script\microsoft.web.webview2.0.9.488\build\native\x64\WebView2Loader.dll build\
for /f "usebackq tokens=*" %%i in (`faust --includedir`) do ^
cl /I build /I "%%i" /I desktop\webview\script /std:c++17 /EHsc /Fobuild\ ^
desktop\main.cpp desktop\webview\script\microsoft.web.webview2.0.9.488\build\native\x64\WebView2Loader.dll.lib ^
/link /OUT:build\groovebox.exe
