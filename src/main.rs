mod shader;

use image;
use shader::Shader;
use std::io::Result;
use std::path::Path;

const DEFAULT_INPUT_PATH: &str = "imgs/input.jpg";
const DEFAULT_OUTPUT_PATH: &str = "imgs/output.jpg";

fn shade(input_path: &str, output_path: &str) {
    let img = image::open(Path::new(input_path)).expect("Failed to open input image");

    Shader::run(img, Path::new(output_path))
}

fn main() -> Result<()> {
    shade(DEFAULT_INPUT_PATH, DEFAULT_OUTPUT_PATH);

    Ok(())
}
