# The Native Code

This directory contains some dummy code that our Rust crate can bind to.

To double-check the code actually works, I've prepared a small "smoke test"
program. This just prepares some inputs and then prints the results.

The `stateful.cpp` file can be compiled directly, or you can use `make` to
build both `stateful.cpp` and the smoke test.

```console
$ make
clang++ --std=c++17 -I. -g -c stateful.cpp
clang++ --std=c++17 -I. -g stateful.o main.cpp -o smoke-test
```

You can then run the test with `make test`

```console
$ make test
./smoke-test
Progress 0%
Progress 25%
Progress 50%
Progress 75%
Progress 100%
Finished with 5 items
0 = -42
1 = 23
2 = 5
3 = 1
4 = 2
```