BUILD_DIR = ${PWD}/build

ifeq ($(shell uname), Darwin)
	PLATFORM_LIBRARIES = -framework WebKit
endif

# TODO Linux
# TODO Windows
# TODO WebAudio/WASM/Emscriptem ?

build/groovebox: $(shell ls native/*.{h,cpp}) native/webview native/miniaudio native/Enfer build/include/effects.h
	time g++ native/desktop.cpp -I native -I build/include -std=c++17 -ldl -lm -lpthread ${PLATFORM_LIBRARIES} -o $@

build/include/effects.h: build/bin/faust native/effects.dsp
	time build/bin/faust -os -ns groovebox -cn Effects -o $@ native/effects.dsp

build/bin/faust: native/faust
	cd native/faust && make PREFIX=${BUILD_DIR} && make install PREFIX=${BUILD_DIR}

native/webview native/miniaudio native/faust native/Enfer:
	git submodule update --init --recursive
