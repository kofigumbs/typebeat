EXECUTABLE_DEPENDENCIES = audio/*.h desktop/main.cpp desktop/webview audio/choc audio/miniaudio audio/Enfer build/Effects.h build/Ui.h
FAUST_INCLUDE = $(shell faust --includedir)

ifeq ($(OS), Windows_NT)
build/groovebox.exe: ${EXECUTABLE_DEPENDENCIES}
	copy desktop\\webview\\dll\\x64\\WebView2Loader.dll build
	cl /I "${FAUST_INCLUDE}" /I desktop\\webview\\script /std:c++17 /EHsc /Fobuild\\ \
		desktop\\main.cpp desktop\\webview\\script\\microsoft.web.webview2.0.9.488\\build\\native\\x64\\WebView2Loader.dll.lib \
		/link /OUT:build\\groovebox.exe
else
PLATFORM_LIBRARIES = $(shell [[ "$$(uname)" == Darwin ]] && echo "-framework WebKit" || pkg-config --cflags --libs gtk+-3.0 webkit2gtk-4.0)
build/groovebox: ${EXECUTABLE_DEPENDENCIES}
	g++ desktop/main.cpp -std=c++17 -ldl -lm -lpthread -I ${FAUST_INCLUDE} ${PLATFORM_LIBRARIES} -o $@
endif

# TODO WebAudio/WASM/Emscriptem ?

build/Ui.h: ui/*
	cat ui/template/1 ui/*.css ui/template/2 ui/*.js ui/template/3 > $@

build/Effects.h: audio/Effects.dsp
	faust -os -cn Effects -o $@ audio/Effects.dsp

desktop/webview audio/choc audio/miniaudio audio/Enfer:
	git submodule init
	git submodule update --recursive
