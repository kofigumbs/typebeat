git submodule init
git submodule update --recursive
faust -os -cn Effects -o build/Effects.h audio/Effects.dsp
copy desktop\webview\script\microsoft.web.webview2.0.9.488\build\native\x64\WebView2Loader.dll build\
for /f "usebackq tokens=*" %%i in (`faust --includedir`) do ^
clang -std=c++17 ^
-D _SILENCE_CLANG_COROUTINE_MESSAGE=1 ^
-I build -I "%%i" -I desktop\webview\script ^
-I "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Tools\MSVC\14.28.29333\include" ^
-I "C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\include\um" ^
-I "C:\Program Files (x86)\Windows Kits\10\include\10.0.18362.0\ucrt" ^
-I "C:\Program Files (x86)\Windows Kits\10\include\10.0.18362.0\shared" ^
-I "C:\Program Files (x86)\Windows Kits\10\include\10.0.18362.0\um" ^
-I "C:\Program Files (x86)\Windows Kits\10\include\10.0.18362.0\winrt" ^
-I "C:\Program Files (x86)\Windows Kits\10\include\10.0.18362.0\cppwinrt" ^
desktop\main.cpp desktop\webview\script\microsoft.web.webview2.0.9.488\build\native\x64\WebView2Loader.dll.lib ^
-o build\groovebox.exe
