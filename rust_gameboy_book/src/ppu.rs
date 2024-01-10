#[derive(Copy, Clone, PartialEq, Eq)]
enum Mode {
    HBlank = 0,
    VBlank = 1,
    OamScan = 2,
    Drawing = 3,
}

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

    vram: Box<[u8; 0x2000]>,
    oam: Box<[u8; 0xA0]>,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            mode: Mode::OamScan,
        }
    }
}
