mod image_manipulator;

use image;
use image_manipulator::ImageManipulator;
use std::io::Result;
use std::path::Path;
mod shaders;
use shaders::NegativeShader;
use shaders::Shader;

const DEFAULT_INPUT_PATH: &str = "imgs/input.jpg";
const DEFAULT_OUTPUT_PATH: &str = "imgs/output.jpg";

fn main() -> Result<()> {
    let input_path = DEFAULT_INPUT_PATH;
    let output_path = DEFAULT_OUTPUT_PATH;
    let shader = &NegativeShader;

    let img = image::open(Path::new(input_path)).expect("Failed to open input image");

    ImageManipulator::run(img, Path::new(output_path), shader);

    Ok(())
}
