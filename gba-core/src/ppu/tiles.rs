use crate::{Ppu, utils::AddressableBits};

impl Ppu {
    pub fn get_tilemap_data(&self, bg_cnt: u16, bgx: u16, bgy: u16) -> u16 {
        let screen_base_block = usize::from(bg_cnt.bits(8, 12)) * 0x800;

        let mut tile_x = bgx / 8;
        let mut tile_y = bgy / 8;
        let screenblock = self.reg_screenblock(tile_x.into(), tile_y.into());
        tile_x %= 32;
        tile_y %= 32;

        let tile_index = usize::from(tile_x + tile_y * 32);
        let tm_data = u16::from_le_bytes([
            self.vram[screen_base_block + 0x800 * screenblock + tile_index * 2],
            self.vram[screen_base_block + 0x800 * screenblock + tile_index * 2 + 1],
        ]);

        tm_data
    }

    pub fn get_pixel_color_offset(&self, bg_cnt: u16, tm_data: u16, bgx: u16, bgy: u16) -> u8 {
        let character_base_block = usize::from(bg_cnt.bits(2, 3)) * 0x4000;

        let flip_vertical = tm_data.bit(11) == 1;
        let flip_horizontal = tm_data.bit(10) == 1;

        let mut subpixel_x = usize::from(bgx % 8);
        if flip_horizontal {
            subpixel_x = 7 - subpixel_x;
        }
        let mut subpixel_y = usize::from(bgy % 8);
        if flip_vertical {
            subpixel_y = 7 - subpixel_y;
        }

        let ts_index: usize = usize::from(tm_data.bits(0, 9));
        let ts_byte = self.vram
            [character_base_block + 32 * ts_index + 4 * subpixel_y + subpixel_x / 2];

        let palette_offset = if subpixel_x % 2 == 0 {
            ts_byte.bits(0, 3)
        } else {
            ts_byte.bits(4, 7)
        };

        palette_offset
    }

    pub fn get_background_pixel(&self, bg: usize) -> u16 {
        let bg_cnt: u16 = self.read_lcd_io_regs(0x4000008 + 2 * bg);
        let scroll_x: u16 = self.read_lcd_io_regs(0x4000010);
        let scroll_y: u16 = self.read_lcd_io_regs(0x4000012);

        let background_x = self.x + scroll_x;
        let background_y = self.lcd_regs.vcount.read() + scroll_y;

        let tm_data = self.get_tilemap_data(bg_cnt, background_x, background_y);

        let color_offset = self.get_pixel_color_offset(bg_cnt, tm_data, background_x, background_y);

        let palette_bank = tm_data.bits(12, 15);
        let color = self.palette_lookup(color_offset.into(), palette_bank.into());

        color
    }
}
