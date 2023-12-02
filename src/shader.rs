use image::Rgba;

pub struct Shader;

impl Shader {
    pub fn shade(pixel: Rgba<u8>) -> Rgba<u8> {
        return Rgba([
            255 - pixel.0[0],
            255 - pixel.0[1],
            255 - pixel.0[2],
            pixel.0[3]
        ]);
    }
}