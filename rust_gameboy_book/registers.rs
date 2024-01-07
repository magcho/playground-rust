#[derive(Clone, Copy, Debug, Default)]

pub struct Registers {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8, // fレジスタの下位4bitは常に0
    pub h: u8,
    pub l: u8,
}
impl Registers {
    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn write_af(&mut self, val: u16) {
        // u16 -> u8のcastはu16の下位8bitをそのまま採用し上位8bitは捨てる

        self.a = (val >> 8) as u8;
        self.f = (val & 0xF0) as u8; // fレジスタの下位4bitは常に0
    }
    pub fn write_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.f = val as u8;
    }
    pub fn write_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }
    pub fn write_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }

    pub fn zf(&self) -> bool {
        // 7654 3210
        // Xxxx xxxx
        // ↑f-z(7bit目 (0based-index)) 演算結果が0の場合1になる

        (self.f & 0b_1000_0000) > 0
    }
    pub fn nf(&self) -> bool {
        // f-n(6bit目) 減算命令の場合１になる
        (self.f & 0b_0100_0000) > 0
    }
    pub fn hf(&self) -> bool {
        // f-h(5bit目) 3bit目で繰り上(下)がりが発生すると1になる
        (self.f & 0b_0010_0000) > 0
    }
    pub fn cf(&self) -> bool {
        // f-h(4bit目) 7bit目で繰り上(下)がりが発生すると1になる
        (self.f & 0b_0001_0000) > 0
    }

    pub fn set_zf(&mut self, zf: bool) {
        if zf {
            self.f |= 0b_1000_0000
        }
    }
    pub fn set_nf(&mut self, nf: bool) {
        if nf {
            self.f |= 0b_0100_0000
        }
    }
    pub fn set_hf(&mut self, hf: bool) {
        if zf {
            self.f |= 0b_0010_0000
        }
    }
    pub fn set_cf(&mut self, cf: bool) {
        if cf {
            self.f |= 0b_0001_0000
        }
    }
}
