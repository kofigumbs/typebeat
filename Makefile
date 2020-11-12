BUILD_DIR = ${PWD}/build

ifeq ($(shell uname), Darwin)
	PLATFORM_LIBRARIES = -framework WebKit
endif

# TODO Linux
# TODO Windows
# TODO WebAudio/WASM/Emscriptem ?

build/groovebox: build/mydsp.o build/desktop.o
	g++ build/mydsp.o build/desktop.o -std=c++17 -ldl -lm -lpthread ${PLATFORM_LIBRARIES} -o $@

build/desktop.o: native/desktop.cpp native/webview native/miniaudio
	g++ -c native/desktop.cpp -I native/faust/architecture -std=c++17 -o $@

build/mydsp.o: build/mydsp.cpp
	g++ -c build/mydsp.cpp -std=c++17 -I native/faust/architecture -o $@ -fbracket-depth=512

build/mydsp.cpp: build/bin/faust dsp/*.dsp
	build/bin/faust -a minimal-effect.cpp -o build/mydsp.cpp dsp/main.dsp

build/bin/faust: native/faust
	cd native/faust && make PREFIX=${BUILD_DIR} && make install PREFIX=${BUILD_DIR}

native/webview native/miniaudio native/faust:
	git submodule update --init --recursive
