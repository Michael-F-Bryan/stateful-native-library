CXX_FLAGS := --std=c++17 -I. -g

build: smoke-test

test: build
	./smoke-test

smoke-test: stateful.o main.cpp
	$(CXX) $(CXX_FLAGS) stateful.o main.cpp -o smoke-test

stateful.o: stateful.h stateful.cpp
	$(CXX) $(CXX_FLAGS) -c stateful.cpp

clean:
	$(RM) *.o
	$(RM) smoke-test

.PHONY: clean test