ifeq ($(OS), Windows_NT)
	EXE=.exe
	CC=cl /std:c++17 /EHsc /c /Fo
	LD=copy vendor\webview\dll\x64\WebView2Loader.dll build && link $(wildcard vendor/webview/script/*/build/native/x64/WebView2Loader.dll.lib) /OUT:
else
	CC=g++ -std=c++17 --output=
	LD=ld -ldl -lm -lpthread $(shell [[ "$$(uname)" == Darwin ]] && echo "-framework WebKit" || pkg-config --cflags --libs gtk+-3.0 webkit2gtk-4.0) --output=
endif

# TODO WebAudio/WASM/Emscriptem ?

build/groovebox${EXE}: build/audio.o build/groovebox.o
	$(LD)$@ $^

build/groovebox.o: audio/audio.h build/Effects.h build/Ui.h desktop/main.cpp | vendor
	$(CC)$@ -I vendor/webview/script desktop/main.cpp

build/audio.o: audio build/Effects.h | vendor
	$(CC)$@ -I "$(shell faust --includedir)" audio/audio.cpp

build/Effects.h: audio/Effects.dsp | build
	faust -os -cn Effects -o $@ audio/Effects.dsp

build:
	mkdir build

vendor:
	git submodule init
	git submodule update --recursive

# hacky, but cross-platform method of templating Ui.h
#
# It works by using Make's info function as a cross-platform echo.
# The following phony targets are "private", and to indicate that, we prefix
# them with -- which requires some extra effort to type properly.
build/Ui.h: ui | build
	make -s -- --UI_HEADER_1 >  $@
	cat ui/*.css             >> $@
	make -s -- --UI_HEADER_2 >> $@
	cat ui/*.js              >> $@
	make -s -- --UI_HEADER_3 >> $@

define UI_HEADER_1
	#include <regex>
	std::string uiHtml() {
	    std::string html = R""""(data:text/html,<!doctype html>
	<meta charset="utf-8">
	<style>
endef
define UI_HEADER_2
	</style>
	<script>
	window.addEventListener('load', function() {
endef
define UI_HEADER_3
	});
	</script>
	)"""";
	    html = std::regex_replace(html, std::regex("%"), "%25");
	    html = std::regex_replace(html, std::regex("\\+"), "%2b");
	    return html;
	}
endef

.PHONY: --UI_HEADER_1 --UI_HEADER_2 --UI_HEADER_3
--UI_HEADER_1:
	$(info $(UI_HEADER_1))
--UI_HEADER_2:
	$(info $(UI_HEADER_2))
--UI_HEADER_3:
	$(info $(UI_HEADER_3))
