BUILD_DIR = ${PWD}/build

ifeq ($(shell uname), Darwin)
	PLATFORM_LIBRARIES = -framework WebKit
endif

# TODO Linux
# TODO Windows
# TODO WebAudio/WASM/Emscriptem ?

.PHONY: all
all: ui/FiraCode build/groovebox

build/groovebox: desktop/main.cpp audio/device.h audio/sequencer.h desktop/webview audio/choc audio/miniaudio audio/Enfer build/include/effects.h
	time g++ desktop/main.cpp -I audio -I build/include -std=c++17 -ldl -lm -lpthread ${PLATFORM_LIBRARIES} -o $@

build/include/effects.h: build/bin/faust audio/effects.dsp
	time build/bin/faust -os -ns groovebox -cn Effects -o $@ audio/effects.dsp

build/bin/faust: audio/faust
	cd audio/faust && make PREFIX=${BUILD_DIR} && make install PREFIX=${BUILD_DIR}

desktop/webview audio/choc audio/faust audio/miniaudio audio/Enfer ui/FiraCode:
	git submodule update --init --recursive
