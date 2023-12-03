use super::shader::Shader;
use image::Rgba;

pub struct PixelShader;

impl Shader for PixelShader {
    fn block_size(&self) -> usize {
        8
    }

    fn shade(&self, block: Vec<(u32, u32, Rgba<u8>)>) -> Vec<(u32, u32, Rgba<u8>)> {
        let mut out_block = vec![];

        // use a reducer to sum values for each colour
        let (r, g, b) = block.iter().fold((0, 0, 0), |(pr, pg, pb), pxl| {
            (
                pr + pxl.2[0] as u32,
                pg + pxl.2[1] as u32,
                pb + pxl.2[2] as u32,
            )
        });
        // calculate the average value for each colour
        let avg = (
            r as u64 / block.len() as u64,
            g as u64 / block.len() as u64,
            b as u64 / block.len() as u64,
        );

        // apply the average colour for all pixels in the block, for a pixelation effect
        let new_rgba = Rgba([avg.0 as u8, avg.1 as u8, avg.2 as u8, 255]);
        for (x, y, _pixel) in block {
            out_block.push((x, y, new_rgba));
        }

        return out_block;
    }
}
