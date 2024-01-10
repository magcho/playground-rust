pub struct Peripherals {
    bootrom: Bootrom,
    wram: WRam,
    hram: HRam,
}
impl Peripherals {
    pub fn new(bootrom: Bootrom) -> Self {
        Self {
            bootrom,
            wram: WRam::new(),
            hram: HRam::new(),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x00FF => {
                if self.bootrom.is_active() {
                    self.bootrom.read(addr)
                } else {
                    0xFF
                }
            }
            0xC000..=0xFDFF => self.wram.read(addr),
            0xFF80..=0xFFFE => self.hram.read(addr),

            0x8000..=0x9FFF => self.ppu.read(addr),
            0xFE00..=0xFE9F => self.ppu.read(addr),
            0xFF40..=0xFF4B => self.ppu.read(addr),

            _ => 0xFF,
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xC000..=0xFDFF => self.wram.write(addr, val),
            0xFF50 => self.bootrom.write(addr, val),
            0xFF80..=0xFFFE => self.hram.write(addr, val),

            0x8000..=0x9FFF => self.ppu.write(addr),
            0xFE00..=0xFE9F => self.ppu.write(addr),
            0xFF40..=0xFF4B => self.ppu.write(addr),

            _ => (),
        }
    }
}
