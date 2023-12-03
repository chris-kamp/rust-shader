use super::shader::Shader;
use image::Rgba;

pub struct NegativeShader;

impl Shader for NegativeShader {
    fn shade(&self, block: Vec<(u32, u32, Rgba<u8>)>) -> Vec<(u32, u32, Rgba<u8>)> {
        let (x, y, pixel) = block[0];

        return vec![(
            x,
            y,
            Rgba([
                255 - pixel.0[0],
                255 - pixel.0[1],
                255 - pixel.0[2],
                pixel.0[3],
            ]),
        )];
    }
}
