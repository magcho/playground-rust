#[derive(Copy, Clone, PartialEq, Eq)]
enum Mode {
    HBlank = 0,
    VBlank = 1,
    OamScan = 2,
    Drawing = 3,
}

const PPU_ENABLE: u8 = 1 << 7;
const WINDOW_TILE_MAP: u8 = 1 << 6;
const WINDOW_ENABLE: u8 = 1 << 5;
const TILE_DATA_ADDRESSING_MODE: u8 = 1 << 4;
const BG_TILE_MAP: u8 = 1 << 3;
const SPRITE_SIZE: u8 = 1 << 2;
const SPRITE_ENABLE: u8 = 1 << 1;
const BG_WINDOW_ENABLE: u8 = 1 << 0;

// lcdc register values
const LYC_EQ_LY_INT: u8 = 1 << 6;
const OAM_SCAN_INT: u8 = 1 << 5;
const VBLANK_INT: u8 = 1 << 4;
const HBLANK_INT: u8 = 1 << 3;
const LYC_EQ_LY: u8 = 1 << 2;

// scy, scx register values
pub struct Ppu {
    mode: Mode,
    lcdc: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,

    stat: u8,

    vram: Box<[u8; 0x2000]>,
    oam: Box<[u8; 0xA0]>,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            mode: Mode::OamScan,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9xFFF => {
                if self.mode == Mode::Drawing {
                    0xFF
                } else {
                    self.vram[addr as usize & 0x1FFF]
                }
            }
            0xFE00..=0xFE9F => {
                if self.mode == Mode::Drawing || self.mode == Mode::OamScan {
                    0xFF
                } else {
                    self.oam[addr as usize & 0xFF]
                }
            }
            0xFF40 => self.lcdc,
            0xFF41 => 0x80 | self.stat | self.mode as u8,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,

            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,

            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x8000..=0x9xFFF => {
                if self.mode != Mode::Drawing {
                    self.vram[addr as usize & 0x1FFF] = val;
                }
            }
            0xFE00..=0xFE9F => {
                if self.mode == Mode::Drawing || self.mode == Mode::OamScan {
                    self.oam[addr as usize & 0xFF] = val;
                }
            }
            0xFF40 => self.lcdc = val,
            0xFF41 => self.stat = (self.stat & LYC_EQ_LY) | (val & 0xF8),
            0xFF42 => self.scy = val,
            0xFF43 => self.scx = val,
            0xFF44 => {} // LYレジスタは書き込み不可
            0xFF45 => self.lyc = val,

            0xFF47 => self.bgp = val,
            0xFF48 => self.obp0 = val,
            0xFF49 => self.obp1 = val,
            0xFF4A => self.wy = val,
            0xFF4B => self.wx = val,

            _ => unreachable!(),
        }
    }

    fn get_pixcel_from_tile(&self, tile_idx: usize, row: u8, col: u8) -> u8 {
        let r = (row * 2) as usize;
        let c = (7 - col) as usize;
        let tile_addr = tile_idx << 4;
        let low = self.vram[(title_addr | r) & 0x1FFF];
        let high = self.vram[(title_addr | (r + 1)) & 0x1FFF];

        (((high >> c) & 1) << 1) | ((l >> c) & 1)
    }

    fn get_tile_idx_from_tile_map(&self, tile_map: bool, row: u8, col: u8) -> usize {
        let start_addr: usize = 0x1800 | ((tile_map as usize) << 10);
        let ret = self.vram[start_addr | (((row as usize) << 5) + col as uisze) & 0x3FF];

        if self.lcdc & TILE_DATA_ADDRESSING_MODE > 0 {
            ret as usize
        } else {
            ((ret as i8 as i16) + 0x100) as usize
        }
    }

    fn render_bg(&mut self) {
        if self.lcdc & BG_WINDOW_ENABLE == 0 {
            return;
        }

        let y = self.y.wrapping_add(self.scy);
        for i in 0..LCD_WIDTH {
            let x = (i as u8).wrapping_add(self.scx);

            let title_idx =
                self.get_tile_idx_from_tile_map(self.lcdc & BG_TILE_MAP > 0, y >> 3, x >> 3);

            let pixcel = self.get_pixcel_from_tile(title_idx, y & 7, x & 7);

            self.buffer[LCD_WIDTH * self.ly as usize + i] = match (self.bgp >> (pixcel << 1)) & 0b11
            {
                b000 => 0xFF, // 白
                0b01 => 0xAA, // ライトグレー
                0b10 => 0xAA, // ダークグレー
                _ => 0x00,    // 黒
            };
        }
    }

    fn check_lyc_eq_ly(&mut self) {
        if self.ly == slef.lyc {
            self.stat |= LYC_EQ_LY;
        } else {
            self.stat &= !LYC_EQ_LY;
        }
    }

    pub fn emulate_cycle(&mut self) -> bool {
        if self.lcdc & PPU_ENABLE == 0
        // PPUが無効化されている場合は何もしない
        {
            return false;
        }

        self.cycles -= 1; // cycleの値を更新する
        if self.cycles > 0 {
            // 最終cycle出ない場合は何もしない
            return false;
        }

        let mut ret = false; // VSYNCであるかを示す変数
        match self.mode {
            Mode::HBlank => {
                self.ly += 1; // HBLANKの終わりは行の終わり何のでLYをインクリメント
                if self.ly < 144 {
                    self.mode = Mode::OamScan; // 描画する行が残っている場合は次のモードはOAM Scan
                    self.cycles = 20;
                } else {
                    // その行がVBlankの手前の行だった場合は次のモードはVBLank
                    self.mode = Mode::VBlank;
                    self.cycles = 114;
                }
                self.check_lyc_eq_ly(); // LYを更新したら必ずLYCと等しいかを確認する
            }
            Mode::VBlank => {
                self.ly += 1; // VBlankの終わりは行の終わりなのでLYをインクリメント
                if self.ly > 153 {
                    // VBlankの最後の行だった場合は次のモードはOAM Scan
                    ret = true; // VBlankの最後はVSYNCのタイミング
                    self.ly = 0; // 先頭の行に戻る
                    self.mode = Mode::OamScan;
                    self.cycles = 20;
                } else {
                    // VBlankの最後の行ではなかった場合はまだVBlank
                    self.cycles = 114;
                }
                self.check_lyc_eq_ly() // LYを更新したら必ずLYCと等しいかを確認する
            }
            Mode::OamScan => {
                // 次のモードはDrawing Pixels
                self.mode = Mode::Drawing;
                self.cycles = 43;
            }
            Mode::Drawing => {
                // 次のモードはHBlank
                self.render_bg(); // Drawing Pixelsの最終cycleなのでレンダリングを実行
                self.mode = Mode::HBlank;
                self.cycles = 51;
            }
        }
    }
}
