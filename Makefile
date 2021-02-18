EXECUTABLE_DEPENDENCIES = audio/*.h desktop/main.cpp desktop/webview audio/choc audio/miniaudio audio/Enfer build/Effects.h build/Ui.h
FAUST_INCLUDE = $(shell faust --includedir)

ifeq ($(OS), Windows_NT)
build/groovebox.exe: ${EXECUTABLE_DEPENDENCIES} build/WebView2Loader.dll
	cl /I "${FAUST_INCLUDE}" /I desktop/webview/script /std:c++17 /EHsc /Fobuild\ \
		desktop/main.cpp $(wildcard desktop/webview/script/*/build/native/x64/WebView2Loader.dll.lib) \
		/link /OUT:build/groovebox.exe
build/WebView2Loader.dll:
	copy desktop\webview\dll\x64\WebView2Loader.dll build
else
PLATFORM_LIBRARIES = $(shell [[ "$$(uname)" == Darwin ]] && echo "-framework WebKit" || pkg-config --cflags --libs gtk+-3.0 webkit2gtk-4.0)
build/groovebox: ${EXECUTABLE_DEPENDENCIES}
	g++ desktop/main.cpp -std=c++17 -ldl -lm -lpthread -I ${FAUST_INCLUDE} ${PLATFORM_LIBRARIES} -o $@
endif

# TODO WebAudio/WASM/Emscriptem ?

build/Effects.h: audio/Effects.dsp
	faust -os -cn Effects -o $@ audio/Effects.dsp

desktop/webview audio/choc audio/miniaudio audio/Enfer:
	git submodule init
	git submodule update --recursive


# hacky, but cross-platform method of templating Ui.h
#
# It works by using Make's info function as a cross-platform echo.
# The following phony targets are "private", and to indicate that, we prefix
# them with -- which requires some extra effort to type properly.

build/Ui.h: ui/*
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
