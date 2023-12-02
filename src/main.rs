use image::{self, GenericImageView, ImageBuffer, Rgba};
use std::path::Path;
use std::io::Result;

fn main() -> Result<()> {
    // Path to the input image
    let input_path = Path::new("imgs/input.jpg");

    // Open the image file
    let img = image::open(input_path).expect("Failed to open input image");

    let (width, height) = img.dimensions();

    let mut output_img = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let new_pixel = Rgba([0, 0, pixel.0[2], 0]);
        output_img.put_pixel(x, y, new_pixel);
    }

    // Path for the output image
    let output_path = Path::new("imgs/output.jpg");

    // Save the image
    output_img.save(output_path).expect("Failed to save output image");

    Ok(())
}