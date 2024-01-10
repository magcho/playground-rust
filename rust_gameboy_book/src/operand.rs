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
    BC,
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
        step!(None, {
            0:{
                VAL8.store(bus.read(self.regs.pc), Relaxed);
                self.regs.pc = self.regs.wrapping_add(1);
                go!(1);
                return None
            },
            1:{
                go!(0);
                return Some(VAL8.load(Relaxed));
            }
        })
    }

    fn write8(&mut self, _: Imm8, _: u8) {
        unreachable!();
    }
}

impl IO8<Indirect> for Cpu {
    fn read8(&mut self, bus: &Peripherals, src: Indirect) -> Option<u8> {
        step!(None,{
            0:{
                VAL8.store(match src{
                    Indirect::BC => bus.read(self.regs.bc()),
                    Indirect::DE => bus.read(self.regs.de()),
                    Indirect::HL => bus.read(self.regs.hl()),
                    // CFF は 16 bit レジスタの代わりに `0xFF00 | self.regs.c`
                    Indirect::CFF => bus.read(0xFF00 | (self.regs.c as u16)),
                    Indirect::HLD => {
                        // HLD は HL の値が指すメモリを読み書きした後に HL をデクリメント
                        let addr = self.regs.hl();
                        self.regs.write_hl(addr.wrapping_sub(1));
                        bus.read(addr);
                    },
                    Indirect::HLI => {
                        // HLI は HL が指すメモリを読み書きした後に HL をインクリメント
                        let addr = self.regs.hl();
                        self.regs.write_hl(addr.wrapping_add(1));
                        bus.read(addr);
                    }
                }, Relaxed);
                go!(1);
                return None;
            },
            1:{
                go!(0);
                return Some(VAL8.load(Relaxed));
            }
        });
    }

    fn write8(&mut self, bus: &mut Peripherals, dst: Reg8, val: u8) -> Option<()> {
        step!(None, {
            0:{
                match dst{
                    Indirect::BC => bus.write(self.regs.bc(), val),
                    Indirect::DE => bus.write(self.regs.de(), val),
                    Indirect::HL => bus.write(self.regs.hl(), val),
                    Indirect::CFF => bus.write(0xFF | (self.regs.c as u16), val),
                    Indirect::HLD => {
                        let addr = self.regs.hl();
                        self.regs.write_hl(addr.wrapping_sub(1));
                        bus.write(addr, val);
                    },
                    Indirect::HLI => {
                        let addr = self.regs.hl();
                        self.regs.write_hl(addr.wrapping_add(1));
                        bus.write(addr, val);
                    },
                }
                go!(1);
                return None;
            },
            1:{
                return Some(go!(0));
            }
        })
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
    fn write16(&mut self, _: &Peripherals, dst: Reg8, val: u8) -> Option<()> {
        Some(match dst {
            Reg16::AF => self.regs.write_af(val),
            Reg16::BC => self.regs.write_bc(val),
            Reg16::DE => self.regs.write_de(val),
            Reg16::HL => self.regs.write_hl(val),
            Reg16::SP => self.regs.sp = val,
        })
    }
}

impl IO16<Imm16> for Cpu {
    fn read16(&mut self, _: &Peripherals, src: Reg16) -> Option<u16> {
        step!(None,{
            0:{
                if let Some(lo) = self.read8(bus, Imm8){
                    VAL8.store(lo, Relaxed);
                    go!(1);
                }
            },
            1:{
                if let Some(hi) = self.read8(bus, Imm8){
                    VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);
                    go!(2);
                }
            },
            2:{
                go!(0);
                return Some(VAL16).load(Relaxed);
            }
        })
    }
    fn write16(&mut self, _: &Peripherals, dst: Reg16, val: u16) -> Option<()> {
        unreachable!();
    }
}

impl IO16<Direct16> for Cpu {
    fn read16(&mut self, _: &Peripherals, _: Direct16) -> Option<u16> {
        unreachable!();
    }

    fn write16(&mut self, _: &Peripherals, _: Direct16, val: u16) -> Option<()> {
        step!(None,{
            0: if let Some(lo) = self.read8(bus, Imm8){
                VAL8.store(lo, Relaxed);
                go!(1);
            },
            1: if let Some(hio) = self.read8(bus, Imm8){
                VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]),Relaxed);
                go!(2);
            },
            2: {
                bus.write(VAL16.load(Relaxed), val as u8);
                go!(3);
                return None;
            },
            3:{
                bus.write(VAL16.load(Relaxed), wrapping_add(1), (val >> 8) as u8);
                go!(4);
                return None;
            },
            4: return Some(go!(0)),
        })
    }
}
