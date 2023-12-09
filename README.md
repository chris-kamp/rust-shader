# rust-shader

A simple command-line tool for applying various pixel shaders to image files,
created as a hobby project mainly for the sake of learning Rust.

## Shaders
Four shaders are available:
- grayscale: Renders the image in grayscale
- negative: Renders the image in negative (inverted colours)
- pixel: Renders the image in a blocky, pixelated style
- palette: Transforms the image to fit a colour palette. Colour palettes are hardcoded for now.

## Usage
To build from source, with Rust installed, run `cargo build --release`.

Usage: rust-shader [OPTIONS] --input <FILE>

Options:
-i, --input <FILE>        Relative path to the input image file
-o, --output <DIRECTORY>  Directory to save the output image file. Defaults to the current working directory. [default: .]
-s, --shader <VALUE>      The shader to apply: <grayscale | negative | palette | pixel>
-a, --all                 Apply all available shaders, generating an output image for each
-h, --help                Print help
-V, --version             Print version
