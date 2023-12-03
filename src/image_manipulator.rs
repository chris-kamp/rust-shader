use super::shaders::Shader;
use image::{DynamicImage, GenericImageView, ImageBuffer};
use std::path::Path;

pub struct ImageManipulator;

impl ImageManipulator {
    pub fn run<S: Shader + ?Sized>(img: &DynamicImage, output_path: &Path, shader: &S) {
        let (width, height) = img.dimensions();
        let mut output_img = ImageBuffer::new(width, height);

        for (x, y, pixel) in img.pixels() {
            output_img.put_pixel(x, y, shader.shade(pixel));
        }

        output_img
            .save(output_path)
            .expect("Failed to save output image");
    }
}
