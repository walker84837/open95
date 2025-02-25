# win95-keygen-rs

This program generates an user-given number of Windows 95 product keys. This 
program is inspired by [alex-free](https://github.com/alex-free/open95keygen)'s 
implementation of the Windows 95 product key generation program in C89.

## Table of Contents

1.  [Installation](#installation)
2.  [Usage](#usage)
3.  [Contributing](#contributing)
4.  [License](#license)
5.  [Acknowledgments](#acknowledgments)
6.  [Contact](#contact)

## Installation

To use this program, you'll need to download Rust at
[rustup.rs](https://rustup.rs/).

You can build and install this program using the following command:

``` console
cargo build --release
```

## Usage

After installing, you can use this program to generate Windows 95 product keys.

  - `--oem`: generate an OEM key [optional: default -> generate a retail key]
  - `-n, --number`: number of product keys to generate [default: 1]

Generate 10 OEM product keys:

``` console
./win95-keygen --oem -n 10
```

Generate 10 retail product keys:

``` console
./win95-keygen -n 10
```

## Contributing

If you'd like to contribute to this project, please follow these guidelines:

1.  Follow the [code of conduct](CODE_OF_CONDUCT.md).

2.  Follow the same code style: format your code using `rustfmt` and use
    `clippy` to catch errors and polish your changes.
    ``` console
    $ rustfmt --edition 2021 src/*
    $ cargo clippy
    ```
3.  For a reliable development experience suitable for production and
    compatibility with the broader ecosystem, use Rust stable instead of Rust
    nightly.

4.  When using external libraries, it is recommended to choose lightweight
    options, such as `ureq` over `reqwest`.

5.  It is recommended to use the standard library instead of creating new
    solutions from scratch.

6.  If proposing changes, such as a new feature, please open an issue.

## License

This project is dual-licensed under the [MIT License](LICENSE_MIT.md) or
[Apache License 2.0](LICENSE_APACHE.md), at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

## Acknowledgments

I'd like to give credit to the following libraries and tools used in this 
project:

  - [clap](https://crates.io/crates/clap) - for creating your command-line
    parser, with all of the bells and whistles, declaratively or procedurally.

  - [fastrand](https://crates.io/crates/fastrand) - for generating random 
    numbers, quickly.

  - [rayon](https://crates.io/crates/rayon) - for simple work-stealing
    parallelism for Rust.

## Contact

If you have any questions or need further assistance, you can contact me at 
<walker84837@gmail.com>.
