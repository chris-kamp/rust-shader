use super::shader::Shader;
use image::Rgba;

pub struct PaletteShader;

impl Shader for PaletteShader {
    fn shade(&self, block: Vec<(u32, u32, Rgba<u8>)>) -> Vec<(u32, u32, Rgba<u8>)> {
        let mut out_block = vec![];

        for (x, y, pixel) in block {
            out_block.push((x, y, self.palette_match(pixel)));
        }

        return out_block;
    }
}

impl PaletteShader {
    fn palette(&self) -> Vec<Rgba<u8>> {
        vec![
            Rgba([0xff, 0xe5, 0xec, 255]),
            Rgba([0xff, 0xc2, 0xd1, 255]),
            Rgba([0xff, 0xb3, 0xc6, 255]),
            Rgba([0xff, 0x8f, 0xab, 255]),
            Rgba([0xfb, 0x6f, 0x92, 255]),
        ]
    }

    fn palette_match(&self, src: Rgba<u8>) -> Rgba<u8> {
        self.palette()
            .iter()
            .min_by_key(|&pc| {
                let dr = pc[0] as i32 - src[0] as i32;
                let dg = pc[1] as i32 - src[1] as i32;
                let db = pc[2] as i32 - src[2] as i32;

                (dr * dr + dg * dg + db * db) as u32
            })
            .copied()
            .unwrap_or_else(|| Rgba([0 as u8, 0 as u8, 0 as u8, 0 as u8]))
    }
}
