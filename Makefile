BUILD_DIR = ${PWD}/build

ifeq ($(shell uname), Darwin)
	PLATFORM_LIBRARIES = -framework WebKit
endif

# TODO Linux
# TODO Windows
# TODO WebAudio/WASM/Emscriptem ?

build/groovebox: engine/desktop.cpp engine/webview engine/miniaudio engine/Enfer build/include/mydsp.h
	g++ engine/desktop.cpp -I build/include -std=c++17 -ldl -lm -lpthread ${PLATFORM_LIBRARIES} -o $@

build/include/mydsp.h: build/bin/faust engine/mydsp.dsp
	build/bin/faust -o $@ engine/mydsp.dsp

build/bin/faust: engine/faust
	cd engine/faust && make PREFIX=${BUILD_DIR} && make install PREFIX=${BUILD_DIR}

engine/webview engine/miniaudio engine/faust engine/Enfer:
	git submodule update --init --recursive
