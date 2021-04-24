ifeq ($(OS), Windows_NT)
	EXE=.exe
	CC=cl -c /std:c++17 /EHsc /Fo
	LD=copy vendor\webview\dll\x64\WebView2Loader.dll build && link $(wildcard vendor/webview/script/*/build/native/x64/WebView2Loader.dll.lib) /OUT:
else
	CC=time g++ -c -std=c++17 --output=
	LD=g++ -ldl -lm -lpthread $(shell [[ "$$(uname)" == Darwin ]] && echo "-framework WebKit" || pkg-config --cflags --libs gtk+-3.0 webkit2gtk-4.0) --output=
endif

# TODO WebAudio/WASM/Emscriptem ?

build/Typebeat${EXE}: build/audio.o build/insert.o build/desktop.o build/base64.o
	$(LD)$@ $^

build/base64.o: .git/modules vendor/cpp-base64/base64.cpp | build
	$(CC)$@ vendor/cpp-base64/base64.cpp

build/desktop.o: .git/modules audio/include/Audio.h audio/include/Effects.h main/desktop.cpp | build
	$(CC)$@ -I "$(shell faust --includedir)" -I vendor/webview/script main/desktop.cpp

build/audio.o: .git/modules audio audio/include/Audio.h
	$(CC)$@ -I "$(shell faust --includedir)" audio/Audio.cpp

build/%.o: build/%.cpp | build
	$(CC)$@ -I "$(shell faust --includedir)" $<

build/%.cpp: audio/effects/%.dsp | build
	faust -a minimal-effect.cpp -cn _$(basename $(notdir $@)) -o $@ $<

build:
	mkdir build

.git/modules:
	git submodule init
	git submodule update --recursive
