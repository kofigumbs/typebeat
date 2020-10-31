SOUL_VERSION = 0.9.58 # see notes/soul-version.md

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
	curl -sSLo build/${SOUL_PATCH_LOADER}.zip https://github.com/soul-lang/SOUL/releases/download/${SOUL_VERSION}/binaries-${SOUL_PLATFORM_NAME}.zip
	unzip -j -d build build/${SOUL_PATCH_LOADER}.zip

dsp/tr808:
	mkdir -p dsp/tr808
	curl -sSLo build/Roland-TR-808.zip https://usercontent.one/wp/www.drumkito.com/wp-content/uploads/2018/12/Roland-TR-808.zip
	unzip -q -j -d dsp/tr808 build/Roland-TR-808.zip
	find . -type f -name '._*' -delete # delete macOS phantom files
