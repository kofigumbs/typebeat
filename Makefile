ifeq ($(OS), Windows_NT)
	EXE=.exe
	CC=cl -c /std:c++17 /EHsc /Fo
	LD=copy vendor\webview\dll\x64\WebView2Loader.dll build && link $(wildcard vendor/webview/script/*/build/native/x64/WebView2Loader.dll.lib) /OUT:
else
	CC=time g++ -c -std=c++17 --output=
	LD=g++ -ldl -lm -lpthread $(shell [[ "$$(uname)" == Darwin ]] && echo "-framework WebKit" || pkg-config --cflags --libs gtk+-3.0 webkit2gtk-4.0) --output=
endif

# TODO WebAudio/WASM/Emscriptem ?

build/groovebox${EXE}: build/audio.o build/desktop.o
	$(LD)$@ $^

build/desktop.o: vendor audio/audio.h desktop/main.cpp | build
	$(CC)$@ -I vendor/webview/script desktop/main.cpp

build/audio.o: vendor audio build/Effects.h
	$(CC)$@ -I "$(shell faust --includedir)" -I audio/faust audio/audio.cpp

build/Effects.h: audio/faust/Effects.dsp | build
	faust -os -cn Effects -o $@ $<

build:
	mkdir build

vendor:
	git submodule init
	git submodule update --recursive
