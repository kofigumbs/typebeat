groovebox: native
	g++ native/desktop.cpp -std=c++17 -ldl -lm -lpthread -framework WebKit -o groovebox
