use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

// 8bitレジスタ
#[derive(Clone, Copy, Debug)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

// 16bitレジスタ，または2つの8bitレジスタからなる16bitの値
#[derive(Clone, Copy, Debug)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

// プログラムカウンタが指す場所から読み取られる8bit
#[derive(Clone, Copy, Debug)]
pub struct Imm8;

// プログラムカウンタが指す場所から読み取られる16bit
#[derive(Clone, Copy, Debug)]
pub struct Imm16;

//16bitレジスタ, または2つの8bitレジスタからなる16bitが指す場所から読み取られる8bit
#[derive(Clone, Copy, Debug)]
pub enum Indirect {
    BD,
    DE,
    HL,
    CFF,
    HLD,
    HLI,
}

// プログラムカウンタが指す場所から読み取られる16bitが指す場所から読み取られる8bit
#[derive(Clone, Copy, Debug)]
pub enum Direct8 {
    D,
    DFF,
}

// プログラムカウンタが指す場所から読み取られる16bitが指す場所から読み取られる16bit
#[derive(Clone, Copy, Debug)]
pub struct Direct16 {}

// フラグレジスタの特定のbit（条件付き実行のために使用される）
#[derive(Clone, Copy, Debug)]
pub enum Cond {
    NZ,
    Z,
    NC,
    C,
}

pub trait IO8<T: Copy> {
    fn read8(&mut self, bus: &Peripherals, src: T) -> Option<u8>;
    fn write8(&mut self, bus: &Peripherals, dst: T, val: u8) -> Option<()>;
}
pub trait IO16<T: Copy> {
    fn read16(&mut self, bus: &Peripherals, src: T) -> Option<u16>;
    fn write16(&mut self, bus: &Peripherals, dst: T, val: u16) -> Option<()>;
}

impl IO8<Reg8> for Cpu {
    fn read8(&mut self, _: &Peripherals, src: Reg8) -> Option<u8> {
        Some(match src {
            Reg8::A => self.regs.a,
            Reg8::B => self.regs.b,
            Reg8::C => self.regs.c,
            Reg8::D => self.regs.d,
            Reg8::E => self.regs.e,
            Reg8::H => self.regs.h,
            Reg8::L => self.regs.l,
        })
    }
    fn write8(&mut self, _: &Peripherals, dst: Reg8, val: u8) -> Option<()> {
        Some(match dst {
            Reg8::A => self.regs.a = val,
            Reg8::B => self.regs.b = val,
            Reg8::C => self.regs.c = val,
            Reg8::D => self.regs.d = val,
            Reg8::E => self.regs.e = val,
            Reg8::H => self.regs.h = val,
            Reg8::L => self.regs.l = val,
        })
    }
}
impl IO8<Imm8> for Cpu {
    fn read8(&mut self, bus: &Peripherals, _: Imm8) -> Option<u8> {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);

        match STEP.load(Relaxed) {
            0 => {
                VAL8.store(bus.read(self.regs.pc), Relaxed);
                self.regs.pc = self.regs.pc.wrapping_add(1);
                STEP.fetch_add(1, Relaxed);
                None
            }
            1 => {
                STEP.store(0, Relaxed);
                Some(VAL8.load(Relaxed))
            }
        }
    }

    fn write8(&mut self, _: Imm8, _: u8) {
        unreachable!();
    }
}

impl IO16<Reg16> for Cpu {
    fn read16(&mut self, _: &Peripherals, src: Reg8) -> Option<u16> {
        Some(match src {
            Reg16::AF => self.regs.af(),
            Reg16::BC => self.regs.bc(),
            Reg16::DE => self.regs.de(),
            Reg16::HL => self.regs.hl(),
            Reg16::SP => self.regs.sp,
        })
    }
    fn write8(&mut self, _: &Peripherals, dst: Reg8, val: u8) -> Option<()> {
        Some(match dst {
            Reg16::AF => self.regs.write_af(val),
            Reg16::BC => self.regs.write_bc(val),
            Reg16::DE => self.regs.write_de(val),
            Reg16::HL => self.regs.write_hl(val),
            Reg16::SP => self.regs.sp = val,
        })
    }
}
