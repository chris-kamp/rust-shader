use super::shader::Shader;
use image::Rgba;
use std::collections::HashMap;

pub struct PaletteShader;

impl Shader for PaletteShader {
    fn shade(&self, block: Vec<(u32, u32, Rgba<u8>)>) -> Vec<(u32, u32, Rgba<u8>)> {
        let mut out_block = vec![];

        for (x, y, pixel) in block {
            out_block.push((x, y, self.palette_match(pixel, self.current_palette())));
        }

        return out_block;
    }
}

impl PaletteShader {
    // Hardcode the palette to use here (might make it configurable later)
    fn current_palette(&self) -> &str {
        "space"
    }

    fn palettes(&self) -> HashMap<&str, Vec<Rgba<u8>>> {
        let mut palettes: HashMap<&str, Vec<Rgba<u8>>> = HashMap::new();

        palettes.insert(
            "pink",
            vec![
                Rgba([0xff, 0xe5, 0xec, 255]),
                Rgba([0xff, 0xc2, 0xd1, 255]),
                Rgba([0xff, 0xb3, 0xc6, 255]),
                Rgba([0xff, 0x8f, 0xab, 255]),
                Rgba([0xfb, 0x6f, 0x92, 255]),
            ],
        );

        palettes.insert(
            "summer",
            vec![
                Rgba([0xfa, 0xf2, 0xd3, 255]),
                Rgba([0xf4, 0xe8, 0x69, 255]),
                Rgba([0x30, 0x85, 0xc3, 255]),
                Rgba([0x5c, 0xd2, 0xe6, 255]),
            ],
        );

        palettes.insert(
            "space",
            vec![
                Rgba([0x0c, 0x13, 0x4f, 255]),
                Rgba([0x1d, 0x26, 0x7d, 255]),
                Rgba([0x5c, 0x46, 0x9c, 255]),
                Rgba([0xd4, 0xad, 0xfc, 255]),
            ],
        );

        palettes
    }

    fn palette_match(&self, src: Rgba<u8>, palette_key: &str) -> Rgba<u8> {
        self.palettes()[palette_key]
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
