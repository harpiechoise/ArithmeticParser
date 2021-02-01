# Arithmetic parser 

A simple project to put in practice the interpreter and compilers principles in rust, this uses the interpreter architecture: AST, Parser, Tokenizer.

The project is documented for study purposses

## Building
To build the project you'll need to install the rustc and the respective dependencies before that you can call cargo run or cargo build and excecute the standalone binary.

```sh
$ git clone https://github.com/harpiechoise/ArithmeticParser.git
$ cd ArithmeticParser
$ cargo run # For run the project
```
Or

```sh
$ git clone https://github.com/harpiechoise/ArithmeticParser.git
$ cd ArithmeticParser
$ cargo build # For build an standalone excecutable
```

## Building the docs
For build the docs you can use the `cargo doc` command

```sh
$ git clone https://github.com/harpiechoise/ArithmeticParser.git
$ cd ArithmeticParser
$ cargo doc # For build the docs
```
And `cargo doc --open` to inspect the documentation

```sh
$ cargo doc --open
```

This will display the documentation in the browser 

## Run the test
For purposes of extending the arithmetic compiler you can run the tests to ensure is all working with the `cargo test` command

```sh
$ cd ArithmeticParser
$ cargo test
```

## Foreword

The integration test aren't written yet, but may be include in futures releases.