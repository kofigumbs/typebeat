BUILD_DIR = ${PWD}/build

ifeq ($(shell uname), Darwin)
	PLATFORM_LIBRARIES = -framework WebKit
endif

# TODO Linux
# TODO Windows
# TODO WebAudio/WASM/Emscriptem ?

build/groovebox: native/desktop.cpp native/webview native/miniaudio build/include/mydsp.h
	g++ native/desktop.cpp -I build/include -std=c++17 -ldl -lm -lpthread ${PLATFORM_LIBRARIES} -o $@

build/include/mydsp.h: build/bin/faust dsp/*.dsp
	build/bin/faust -o $@ dsp/main.dsp

build/bin/faust: native/faust
	cd native/faust && make PREFIX=${BUILD_DIR} && make install PREFIX=${BUILD_DIR}

native/webview native/miniaudio native/faust:
	git submodule update --init --recursive
