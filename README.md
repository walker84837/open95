# win95-keygen-rs

This program generates an user-given number of Windows 95 product keys. This program is inspired by [alex-free](https://github.com/alex-free/open95keygen)'s implementation of the Windows 95 product key validation program in C.

## Table of Contents

1.  [Installation](#installation)
2.  [Usage](#usage)
3.  [Contributing](#contributing)
4.  [License](#license)
5.  [Acknowledgments](#acknowledgments)
6.  [Contact](#contact)

## Installation

To use this program, you'll need to install Rust and its package manager, Cargo. Follow the official [Rust installation guide](https://www.rust-lang.org/tools/install) to get them set up.

Once Rust and Cargo are installed, you can build and install this program using the following command:

``` bash
cargo install --path .
```

## Usage

After installing, you can use this program to generate Windows 95 product keys.

``` bash
./win95-keygen-rs <-o|--oem> <-q> [number]
```

If `-o` or `--oem` isn't provided, the program will generate a retail key. You need to provide `-q` or `--quantity` to provide the number of product keys you'd like to generate.

Here are some examples on how to use it.

1.  Generate 10 OEM product keys:

``` bash
./win95-keygen-rs -o -q 10
```

2.  Generate 10 retail product keys:

``` bash
./win95-keygen-rs -q 10
```

## Contributing

If you'd like to contribute to this project, please follow these guidelines:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix: `git checkout -b feature/new-feature`.
3.  Make your changes and commit them.
4.  Push your changes to your fork: `git push origin feature/new-feature`.
5.  Create a pull request to the `main` branch of the original repository.

## License

This project is dual-licensed under the [MIT License](https://mit-license.org/) and [Apache License 2.0](https://apache.org/licenses/LICENSE-2.0).

## Acknowledgments

I'd like to give credit to the following libraries and tools used in this project:

  - [StructOpt](https://crates.io/crates/structopt) - for command-line argument parsing in Rust.
  - [fastrand](https://crates.io/crates/fastrand) - for generating random numbers, quickly.

## Contact

If you have any questions or need further assistance, you can contact me at <walker84837@gmail.com>.
