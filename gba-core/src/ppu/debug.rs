use crate::ppu::Ppu;
use crate::GbaCore;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::Clamped;

#[wasm_bindgen]
/// A collection of colors that make up the 8x8 tile
/// Each pixel is called a dot
pub struct Tile {
    #[wasm_bindgen(skip)]
    pub palette_offsets: Vec<u8>,
}

#[wasm_bindgen]
impl Tile {
}

impl Tile {
    pub fn from_16_color_data(data: &[u8]) -> Self {
        assert_eq!(data.len(), 32);
        let palette_offsets = data.iter().flat_map(|b| [b & 0xf, b >> 4]).collect();
        Self {
            palette_offsets
        }
    }
    pub fn from_256_color_data(data: &[u8]) -> Self {
        assert_eq!(data.len(), 64);
        Self { palette_offsets: data.to_vec() }
    }
}

#[cfg(feature = "debugger")]
#[wasm_bindgen]
impl Ppu {
    pub fn bg_tilemap(&self, index: usize) -> Vec<u8> {
        let num_tiles = 16;
        let mut bytes: Vec<u8> = vec![];

        for ts_index in 0..num_tiles {
            for subtile_x in 0..8 {
                for subtile_y in 0..8 {
                    bytes.extend_from_slice(
                        &self.get_tile_color(index, ts_index, subtile_x, subtile_y),
                    );
                }
            }
        }

        bytes
    }

    /// Returns a vector of the tiles stored in VRAM, interpreting their bytes based on the given
    /// parameters.
    pub fn debug_tiles(&self, more_colors: bool) -> Vec<Tile> {
        let map_and_tiles = &self.vram[..=0xffff];
        if more_colors {
            map_and_tiles
                .chunks_exact(64)
                .map(Tile::from_256_color_data)
                .collect()
        } else {
            map_and_tiles
                .chunks_exact(32)
                .map(Tile::from_16_color_data)
                .collect()
        }
    }
}

#[cfg(feature = "debugger")]
#[wasm_bindgen]
impl GbaCore {
    pub fn debug_bg_tilemap(&self, index: usize) -> Vec<u8> {
        self.bus.ppu.bg_tilemap(index)
    }
    
    fn decode_16(&self, palette16: usize, offset: u8) -> [u8; 4] {
        let color = self.bus.ppu.palette_lookup_16(palette16, offset.into());
        let alpha = if offset == 0 {
            0
        } else {
            255
        };
        [color[0], color[1], color[2], alpha]
    }

    fn decode_256(&self, offset: u8) -> [u8; 4] {
        let color = self.bus.ppu.palette_lookup_256(offset.into());
        let alpha = if offset == 0 {
            0
        } else {
            255
        };
        [color[0], color[1], color[2], alpha]
    }

    pub fn draw_tiles(
        &self,
        ctx: &web_sys::CanvasRenderingContext2d,
        palette16: Option<usize>,
    ) -> Result<(), wasm_bindgen::JsValue> {
        let tiles = self.bus.ppu.debug_tiles(palette16.is_none());
        let canvas = ctx
            .canvas()
            .ok_or_else(|| js_sys::Error::new("Context must have a canvas"))?;
        let width = canvas.width();


        let mut row = 0;
        let mut col = 0;
        for tile in &tiles {
            let unclamped_data: Vec<u8> = if let Some(palette16) = palette16 {
                tile
                .palette_offsets
                .iter()
                .flat_map(|&offset| self.decode_16(palette16, offset))
                .collect()
            } else {
                tile
                .palette_offsets
                .iter()
                .flat_map(|&offset| self.decode_256(offset))
                .collect()
            };

            let clamped_data: Clamped<&[u8]> = Clamped(&unclamped_data);
            let image_data = web_sys::ImageData::new_with_u8_clamped_array(clamped_data, 8)?;
            ctx.put_image_data(&image_data, col.into(), row.into())?;

            col += 8;
            if col + 8 > width {
                col = 0;
                row += 8;
            }
        }

        Ok(())
    }
}
