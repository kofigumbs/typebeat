groovebox: desktop
	g++ desktop/main.cpp -std=c++17 -ldl -lm -lpthread -framework WebKit -o groovebox
