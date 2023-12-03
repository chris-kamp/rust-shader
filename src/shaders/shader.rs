use image::Rgba;

pub trait Shader {
    fn shade(&self, block: Vec<(u32, u32, Rgba<u8>)>) -> Vec<(u32, u32, Rgba<u8>)>;

    fn block_size(&self) -> usize {
        1
    }
}
