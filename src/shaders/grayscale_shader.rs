use super::shader::Shader;
use image::Rgba;

pub struct GrayscaleShader;

impl Shader for GrayscaleShader {
    fn shade(&self, block: Vec<(u32, u32, Rgba<u8>)>) -> Vec<(u32, u32, Rgba<u8>)> {
        let (x, y, pixel) = block[0];

        let avg = ((pixel.0[0] as u32 + pixel.0[1] as u32 + pixel.0[2] as u32) / 3) as u8;
        return vec![(x, y, Rgba([avg, avg, avg, pixel.0[3]]))];
    }
}
