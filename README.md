# Sflynlang Interpreter

![Rust CI (Workflow)](https://img.shields.io/github/workflow/status/sflynlang/sflynlang-interpreter/Rust%20CI?label=Rust%20CI)
[![Twitter Followers](https://img.shields.io/twitter/follow/sflynlang?style=social)](https://twitter.com/sflynlang)

This project contains the source code of the interpreter for the Sflynlang programming language. It is written in Rustlang.

## Pre-requisites
- [Rustlang](https://rustup.rs/)

## Structure of Directories
In this project you'll find 4 principal directories:

1. **Compiler**: Here is allowed the typechecking and compiling steps of the Sflynlang interpreter.

2. **Core**: Here is allowed the CLI of Sflynlang built using [clap-rs](https://github.com/clap-rs/clap).

3. **Parser**: Here is allowed the lexing and parsing steps of the Sflynlang interpreter.

4. **Examples**: Here is allowed the examples of Sflynlang code.

## Compiling
1. Download the [pre-requisites](#Pre-requisites).

2. Clone this repository.

- **GitHub CLI**: `gh repo clone sflynlang/sflynlang-interpreter`
- **Git**: `git clone https://github.com/sflynlang/sflynlang-interpreter.git`

3. Go to the project directory and then run `cargo build --release`.

4. Congratulations! You've compiled the Sflynlang interpreter project, now you find a binary file (`.exe` on Windows) in `PROJECT_DIRECTORY/target/release/sflyn`.

## Testing
To check if the code works fine, we build tests of our code and to test them use `cargo test` (Add `--release` flag to run in a production context).

Also, this command is used by the `Rust CI` workflow to check if the code to merge works.

## Social Networks
- [Twitter](https://twitter.com/sflynlang)
- [Facebook](https://facebook.com/sflynlang)

## Contributors
- **Daniel Solarte** - Initial Work - [GitHub](https://github.com/danielsolartech)
- **Maria Antonella** - Icon Design - [Instagram](https://instagram.com/elementalnsky)
- **Jheyson Saavedra** - Docker - [GitHub](https://github.com/JheysonDev)

You can also view the [contributors list](https://github.com/sflynlang/sflynlang-interpreter/contributors) here.

## Licensing
This project is under the MIT license. See the [LICENSE](./LICENSE) file for more information.
