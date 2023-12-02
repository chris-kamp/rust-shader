use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::path::Path;

pub struct ImageManipulator;

impl ImageManipulator {
    pub fn run(img: DynamicImage, output_path: &Path) {
        let (width, height) = img.dimensions();
        let mut output_img = ImageBuffer::new(width, height);

        for (x, y, pixel) in img.pixels() {
            output_img.put_pixel(x, y, ImageManipulator::shade(pixel));
        }

        output_img
            .save(output_path)
            .expect("Failed to save output image");
    }

    fn shade(pixel: Rgba<u8>) -> Rgba<u8> {
        return Rgba([
            255 - pixel.0[0],
            255 - pixel.0[1],
            255 - pixel.0[2],
            pixel.0[3],
        ]);
    }
}