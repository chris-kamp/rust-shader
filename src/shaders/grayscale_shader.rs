use super::shader::Shader;
use image::Rgba;

pub struct GrayscaleShader;

impl Shader for GrayscaleShader {
    fn shade(&self, pixel: Rgba<u8>) -> Rgba<u8> {
        let avg = ((pixel.0[0] as u32 + pixel.0[1] as u32 + pixel.0[2] as u32) / 3) as u8;
        return Rgba([avg, avg, avg, pixel.0[3]]);
    }
}
