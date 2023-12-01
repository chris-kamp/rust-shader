use image; // Import the image crate
use std::path::Path;
use std::io::Result;

fn main() -> Result<()> {
    // Path to the input image
    let input_path = Path::new("imgs/input.jpg");

    // Open the image file
    let img = image::open(input_path).expect("Failed to open input image");

    // Path for the output image
    let output_path = Path::new("imgs/output.jpg");

    // Save the image
    img.save(output_path).expect("Failed to save output image");

    Ok(())
}