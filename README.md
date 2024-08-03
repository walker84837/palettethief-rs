# PaletteThief

Pallet extracts colours from images. It uses the
[color-thief](https://crates.io/crates/color-thief) library to create a palette
of dominant colours.

## Table of Contents

  - [Features](#features)
  - [Installation](#installation)
      - [Build from Source](#build-from-source)
      - [Arch Linux](#arch-linux)
  - [Usage](#usage)
      - [Basic Usage](#basic-usage)
      - [Options](#options)
      - [Example](#example)
  - [Contributing](#contributing)
  - [License](#license)
  - [Project Status](#project-status)

## Features

  - **Color Extraction**: Extract a color palette from any image.
  - **Quality Control**: Adjust the quality of the color extraction process.
  - **Custom Palette Size**: Specify the maximum number of colors in the
    palette.
  - **Output Formats**: Colors can be output in either RGB or hexadecimal
    format.
  - **Verbose Mode**: Optional verbose logging for debugging and detailed
    information.
  - **Custom Prefix**: Add a prefix to the color output for easier readability.

## Installation

### Build from Source

Requirements:

  - **Rust**: You can install it using [Rustup](https://rustup.rs/).

<!-- end list -->

1.  Clone the repository:
    
    ``` console
    git clone https://github.com/walker84837/palettethief-rs.git
    cd palettethief-rs
    ```

2.  Build the project:
    
    ``` console
    cargo build --release
    ```

3.  The compiled binary will be located in the `target/release` directory.

### Arch Linux

For Arch Linux users, I have created a PKGBUILD file to make it easier to
install the package. The PKGBUILD can be found in the archlinux folder at the
root of this git repository.

I will follow the steps on ArchWiki for posting packages to the AUR, as I will
be uploading PKGBUILDs to the AUR in the future, making it even easier for end
users to download and install my packages.

## Usage

### Basic Usage

To extract a color palette from an image, run the following command:

``` console
$ palthief <IMAGE_PATH>
```

### Options

  - `-q`, `--quality`: Set the quality of the palette extraction (1..10). Higher
    quality might increase processing time. **Default is 10.**
  - `-m`, `--max_colors`: Set the maximum number of colors in the palette
    (2..255). **Default is 6.**
  - `-v`, `--verbose`: Enable verbose mode to get detailed logs of the
    extraction process.
  - `--rgb`: Output colors in RGB format instead of the default hexadecimal
    format.
  - `--prefix <PREFIX>`: Add a prefix to the color output (e.g., `"Color"` will
    format the output as `Color 1: #000000`).

### Example

Extract a palette with a maximum of 8 colors, using a quality setting of 8, and
output the colors in RGB format:

``` console
$ palthief path/to/image.png -q 8 -m 8 --rgb
```

Enable verbose mode and add a custom prefix to the output:

``` console
$ palthief path/to/image.png -v --prefix "Palette Color"
```

## Contributing

Contributions are welcome! Please submit a pull request or open an issue to
discuss the changes you want to make.

To get started:

1.  Fork the repository.
2.  Create a new branch for your feature or bugfix.
3.  Make your changes.
4.  Run tests to ensure everything is working.
5.  Submit a pull request.

## License

This project is dual-licensed under the [MIT License](LICENSE_MIT.md) or the
[Apache v2 License](LICENSE_APACHE.md).

## Project Status

This project is actively maintained. Future updates may include additional
output formats and performance improvements. Comments and suggestions are always
welcome.
