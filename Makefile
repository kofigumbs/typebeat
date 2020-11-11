BUILD_DIR = ${PWD}/build

ifeq ($(shell uname), Darwin)
	PLATFORM_LIBRARIES = -framework WebKit
endif

# TODO Linux
# TODO Windows
# TODO WebAudio/WASM/Emscriptem ?

build/groovebox: build/mydsp.cpp native/desktop.cpp native/webview native/miniaudio
	g++ build/mydsp.cpp native/desktop.cpp -std=c++17 -ldl -lm -lpthread ${PLATFORM_LIBRARIES} -I native/faust/architecture -o $@

build/mydsp.cpp: build/bin/faust dsp/*.dsp
	build/bin/faust -a minimal-effect.cpp -o $@ dsp/main.dsp

build/bin/faust: native/faust
	cd native/faust && make PREFIX=${BUILD_DIR} && make install PREFIX=${BUILD_DIR}

native/webview native/miniaudio native/faust:
	git submodule update --init --recursive
