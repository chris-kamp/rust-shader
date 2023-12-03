mod image_manipulator;

use image;
use image_manipulator::ImageManipulator;
use std::collections::HashMap;
use std::io::Result;
use std::path::Path;
mod shaders;
use shaders::GrayscaleShader;
use shaders::NegativeShader;
use shaders::PaletteShader;
use shaders::PixelShader;
use shaders::Shader;

const DEFAULT_INPUT_PATH: &str = "imgs/input.jpg";

fn main() -> Result<()> {
    let input_path = DEFAULT_INPUT_PATH;
    let img = image::open(Path::new(input_path)).expect("Failed to open input image");

    let mut shaders: HashMap<&str, Box<dyn Shader>> = HashMap::new();
    shaders.insert("grayscale", Box::new(GrayscaleShader));
    shaders.insert("negative", Box::new(NegativeShader));
    shaders.insert("pixel", Box::new(PixelShader));
    shaders.insert("palette", Box::new(PaletteShader));

    for (&key, shader) in &shaders {
        let output_file = format!("imgs/output-{}.jpg", key);
        ImageManipulator::run(&img, Path::new(&output_file), shader.as_ref());
    }

    Ok(())
}
