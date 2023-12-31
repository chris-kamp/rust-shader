mod image_manipulator;

use image;
use image_manipulator::ImageManipulator;
use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::Path;
mod shaders;
use shaders::GrayscaleShader;
use shaders::NegativeShader;
use shaders::PaletteShader;
use shaders::PixelShader;
use shaders::Shader;

use clap::{arg, command, value_parser, ArgMatches};
use std::path::PathBuf;

fn get_args() -> ArgMatches {
    command!()
        .arg(
            arg!(
                -i --input <FILE> "Relative path to the input image file"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        ).arg(
        arg!(
                -o --output <DIRECTORY> "Directory to save the output image file. Defaults to the current working directory."
            )
            .value_parser(value_parser!(PathBuf))
            .default_value("."),
        ).arg(
            arg!(
                -s --shader <VALUE> "The shader to apply: <grayscale | negative | palette | pixel>"
            )
            .required(false),
        )
        .arg(arg!(
            -a --all "Apply all available shaders, generating an output image for each"
        ))
        .get_matches()
}

fn prepare_shaders() -> HashMap<String, Box<dyn Shader>> {
    let mut shaders: HashMap<String, Box<dyn Shader>> = HashMap::new();
    shaders.insert("grayscale".to_string(), Box::new(GrayscaleShader));
    shaders.insert("negative".to_string(), Box::new(NegativeShader));
    shaders.insert("pixel".to_string(), Box::new(PixelShader));
    shaders.insert("palette".to_string(), Box::new(PaletteShader));

    shaders
}

fn print_results(generated: &[String], output_directory: &PathBuf) {
    let plural_suffix = if generated.len() == 1 { "" } else { "s" };

    if let Ok(absolute_path) = fs::canonicalize(output_directory) {
        print!(
            "Success! Generated output file{} in {}:",
            plural_suffix,
            absolute_path.display()
        );
    } else {
        print!(
            "Success! Generated output file{} in {}:",
            plural_suffix,
            output_directory.display()
        );
    }

    for (idx, file) in generated.iter().enumerate() {
        if idx != 0 && idx != generated.len() {
            print!("|")
        }
        print!(" {} ", file);
    }
    println!("");
}

fn main() -> Result<()> {
    let shaders = prepare_shaders();
    let arg_matches = get_args();

    let input_path;
    if let Some(val) = arg_matches.get_one::<PathBuf>("input") {
        input_path = val;
    } else {
        eprintln!("Error: Missing required argument. Specify an input image file with --input <FILE>. Call with --help for more information.");
        std::process::exit(1);
    }

    let output_directory;
    if let Some(val) = arg_matches.get_one::<PathBuf>("output") {
        output_directory = val;
    } else {
        eprintln!("An unexpected error occurred. Expected --output arg to have a default value, but found none.");
        std::process::exit(1);
    }

    let shader_key: &str;
    if arg_matches.get_flag("all") {
        shader_key = "all";
    } else if let Some(val) = arg_matches.get_one::<String>("shader") {
        shader_key = val;
    } else {
        eprintln!("Error: Missing required argument. Specify a shader with -s <shader>, or pass -a to generate an output file for each available shader. Call with --help for more information.");
        std::process::exit(1);
    }

    let img = image::open(Path::new(input_path)).expect("Failed to open input image");
    let mut generated = vec![];
    for (key, shader) in &shaders {
        if shader_key != "all" && key != shader_key {
            continue;
        }

        let mut output_path = PathBuf::from(output_directory);
        output_path.push(format!("output-{}.jpg", key));

        ImageManipulator::run(&img, Path::new(&output_path), shader.as_ref());

        generated.push(format!("{}.jpg", key));
    }

    print_results(&generated, output_directory);

    Ok(())
}
