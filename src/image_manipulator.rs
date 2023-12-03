use super::shaders::Shader;
use image::{DynamicImage, GenericImageView, ImageBuffer};
use std::path::Path;

pub struct ImageManipulator;

impl ImageManipulator {
    pub fn run<S: Shader + ?Sized>(img: &DynamicImage, output_path: &Path, shader: &S) {
        let (width, height) = img.dimensions();
        let mut output_img = ImageBuffer::new(width, height);
        let block_size = shader.block_size();

        for yblock in (0..height).step_by(block_size) {
            for xblock in (0..width).step_by(block_size) {
                let mut block = vec![];

                for y in yblock..(std::cmp::min(yblock + block_size as u32, height)) {
                    for x in xblock..(std::cmp::min(xblock + block_size as u32, width)) {
                        block.push((x, y, img.get_pixel(x, y)))
                    }
                }

                for (sx, sy, spx) in shader.shade(block) {
                    output_img.put_pixel(sx, sy, spx);
                }
            }
        }

        output_img
            .save(output_path)
            .expect("Failed to save output image");
    }
}
