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

use clap::{arg, command, value_parser};
use std::path::PathBuf;

fn main() -> Result<()> {
    let matches = command!()
        .arg(
            arg!(
                -i --input <FILE> "Relative path to the input image file"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -s --shader "The shader to apply (if not passed, an output image will be generated for each shader): <grayscale | negative | palette | pixel>"
        ))
        .get_matches();

    let input_path = matches
        .get_one::<PathBuf>("input")
        .expect("Missing required argument --input <FILE>");
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
