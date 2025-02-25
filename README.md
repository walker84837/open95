# Open95

This program generates a user-specified number of Windows 95 product keys. It is inspired by [alex-free's](https://github.com/alex-free/open95keygen) implementation of the Windows 95 product key generation program in C89.

## Table of Contents

1. [Installation](#installation)
2. [Usage](#usage)
3. [Contributing](#contributing)
4. [License](#license)
5. [Acknowledgments](#acknowledgments)
6. [Contact](#contact)

## Installation

To use this program, you'll need to have Rust installed. You can download Rust from the [rustup.rs](https://rustup.rs/) website.

Once you have Rust installed, you can build the program using the following command:

```console
cargo build --release
```

## Usage

After installing, you can use this program to generate Windows 95 product keys. The program supports the following options:

- `--oem`: Generate an OEM key (optional, default is to generate a retail key)
- `-n, --number`: The number of product keys to generate (default is 1)

To generate 10 OEM product keys, use the following command:

```console
./open95 --oem -n 10
```

To generate 10 retail product keys, use the following command:

```console
./open95 -n 10
```

## Contributing

Contributions are welcome! When making changes, remember to format your code using `cargo fmt`.

### Contact

If you have any questions or need further assistance, you can open an issue.

## License

This project is dual-licensed under the [MIT License](LICENSE_MIT.md) or [Apache License 2.0](LICENSE_APACHE.md), at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgments

I'd like to give credit to the following libraries and tools used in this project:

- [clap](https://crates.io/crates/clap) - for creating your command-line parser, with all of the bells and whistles, declaratively or procedurally.
- [fastrand](https://crates.io/crates/fastrand) - for generating random numbers, quickly.
- [rayon](https://crates.io/crates/rayon) - for simple work-stealing parallelism for Rust.
