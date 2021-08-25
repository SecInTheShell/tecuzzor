all: server

server: server.cpp
	g++ $^ -o $@

test: test-cli

test-cli: test-cli.cpp
	g++ $^ -o $@

clean:
	rm -f server test-cli
