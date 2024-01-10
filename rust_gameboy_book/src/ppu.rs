#[derive(Copy, Clone, PartialEq, Eq)]
enum Mode {
    HBlank = 0,
    VBlank = 1,
    OamScan = 2,
    Drawing = 3,
}

pub struct Ppu {
    mode: Mode,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            mode: Mode::OamScan,
        }
    }
}
