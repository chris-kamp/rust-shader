use image::Rgba;

pub trait Shader {
    fn shade(&self, pixel: Rgba<u8>) -> Rgba<u8>;
}
