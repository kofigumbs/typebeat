SOUL_VERSION = 0.9.59 # see notes/soul-version.md

ifeq ($(shell uname), Darwin)
	PLATFORM_LIBRARIES = -framework WebKit
	SOUL_PLATFORM_NAME = osx-x64
	SOUL_PATCH_LOADER = SOUL_PatchLoader.dylib
endif

# TODO Linux
# TODO Windows
# TODO WebAudio/WASM/Emscriptem ?

build/groovebox: native/webview native/miniaudio native/SOUL native/desktop.cpp build/${SOUL_PATCH_LOADER}
	g++ native/desktop.cpp -std=c++17 -ldl -lm -lpthread ${PLATFORM_LIBRARIES} -o build/groovebox

native/webview native/miniaudio native/SOUL:
	git submodule update --init

build/soul build/${SOUL_PATCH_LOADER}:
	mkdir -p build
	curl -sSLo build/${SOUL_PATCH_LOADER}.zip https://github.com/soul-lang/SOUL/releases/download/$(strip ${SOUL_VERSION})/binaries-${SOUL_PLATFORM_NAME}.zip
	unzip -j -d build build/${SOUL_PATCH_LOADER}.zip
