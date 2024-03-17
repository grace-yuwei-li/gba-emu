use crate::ppu::Ppu;
use crate::GbaCore;
use wasm_bindgen::prelude::wasm_bindgen;

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
}

#[cfg(feature = "debugger")]
#[wasm_bindgen]
impl GbaCore {
    pub fn bg_tilemap(&self, index: usize) -> Vec<u8> {
        self.bus.ppu.bg_tilemap(index)
    }
}
