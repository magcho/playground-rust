impl Cpu {
    pub fn nop(&mut self, bus: &Peripherals) {
        self.fetch(bus);
    }

    pub fn ld<D: Copy, S: Copy>(&mut self, bus: &mut Peripherals, dst: D, src: S)
    where
        Self: IO8<D> + IO8<S>,
    {
        step!((), {
            0: if let Some(v) = self.read8(bus, src){
                VAL8.store(v, Relaxed);
                go!(1);
            },
            1: if let self.write8(bus, dst, VAL8.load(Relaxed)).is_some(){
              go!(2);
            },
            2:{
                go!(0);
                self.fetch(bus);
            }
        })
    }

    pub fn ld16<D: Copy, S: Copy>(&mut self, bus: &Peripherals, dst: D, src: S)
    where
        Self: IO16<D> + IO16<S>,
    {
        step!((),{
            0: if let Some(v) = self.read16(bus, src){
                VAL16.store(v, Relaxed);
                go!(1);
            },
            1: self.write16(bus, dst, VAL16.load(Relaxed)).is_some(){
                go!(2);
            },
            2: {
                go!(0);
                return self.fetch(bus);
            }
        })
    }

    // A レジスタと s の値の比較
    pub fn cp<S: Copy>(&mut self, bus: &Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        if let Some(v) = self.read8(bus, src) {
            let (result, carry) = self.regs.a.overflowing_sub(v);

            self.regs.set_zf(result == 0);
            self.regs.set_nf(true);
            self.regs.set_hf((self.regs.a & 0xf) < (v & 0xf));
            self.regs.set_cf(carry);
        }
    }

    pub fn inc<S: Copy>(&mut self, bus: &Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        step!((),{
            0: if let Some(v) = self.read8(bus, src){
                let result = v.wrapping_add(1);

                self.regs.set_zf(result == 0);
                self.regs.set_nf(false);
                self.regs.set_hf(v & 0xf == 0xf);

                VAL8.store(result, Relaxed);
                go!(1);
            },
            1: if self.write8(bus,src,VAL8.load(Relaxed)).is_some(){
                go!(0);
                self.fetch(bus);
            }
        })
    }

    pub fn inc16<S: Copy>(&mut self, bus: &Peripherals, src: S)
    where
        Self: IO16<s>,
    {
        step!((),{
            0: if let Some(v) = self.read16(bus, src){
                VAL16.store(v.wrapping_add(1), Relaxed);
                go!(1);
            },
            1: if self.write16(bus, src, VAL16.load(Relaxed)).is_some(){
                go!(2);
            },
            2: {
                go!(0);
                self.fetch(bus);
            }
        })
    }

    pub fn dec<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        step!((),{
            0: if let Some(v) = self.read8(bus, src){
                let result = v.wrapping_sub(1);

                self.regs.set_zf(result == 0);
                self.regs.set_nf(true);
                self.regs.set_hf(v & 0xf == 0);

                VAL8.store(result, Relaxed);
                go!(1);
            },
            1: if self.write8(bus,src,VAL8.load(Relaxed)).is_some(){
                go!(0);
                self.fetch(bus);
            }

        })
    }

    pub fn dec16<S: Copy>(&mut self, bus: &Peripherals, src: S)
    where
        Self: IO16<s>,
    {
        step!((),{
            0: if let Some(v) = self.read16(bus, src){
                VAL16.store(v.wrapping_sub(1), Relaxed);
                go!(1);
            },
            1: if self.write16(bus, src, VAL16.load(Relaxed)).is_some(){
                go!(2);
            },
            2: {
                go!(0);
                self.fetch(bus);
            }
        })
    }

    pub fn rl<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        step!((),{
            0: if let Some(v)= self.read8(bus,src){
                let result = (v << 1) | self.regs.cf() as u8;

                self.regs.set_zf(result == 0);
                self.regs.set_nf(false);
                self.regs.set_hf(false);
                self.regs.set_cf(v & 0x80 > 0);

                VAL8.store(result, Relaxed);
                go!(1);
            },
            1: if self.write8(bus, src, VAL8.load(Relaxed)).is_some(){
                go!(0);
                self.fetch(bus);
            }
        })
    }

    pub fn bit<S: Copy>(&mut self, bus: &Peripherals, bit: usize, src: S)
    where
        Self: IO8<S>,
    {
        if let Some(mut v) = self.read8(bus, src) {
            v &= 0x1 << bit;

            self.regs.set_zf(v == 0);
            self.regs.set_nf(false);
            self.regs.set_hf(true);

            self.fetch(bus);
        }
    }

    pub fn push16(&mut self, bus: &mut Peripherals, val: u16) -> Option<()> {
        step!(None,{
            0: {
                go!(1);  // cycle数はメモリアクセス数+1
                return None;
            },
            1:{
                let [lo, hi] = u16::to_le_bytes(val);
                self.regs.sp = self.regs.sp.wrapping_sub(1); // GBのスタックは上方向(ポインターが減る方向)に積まれる
                bus.write(self.regs.sp, hi);
                VAL8.store(lo, Relaxed);
                go!(2);
                return None;
            },
            2:{
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                bus.write(self.regs.sp, VAL8.load(Relaxed));
                go!(3);
                return None;
            },
            3:{
                return Some(go!(0));
            }
        })
    }

    pub fn push(&mut self, bus: &mut Peripherals, src: Reg16) {
        step!((), {
            0: {
                VAL16.store(self.read16(bus, src).unwrap(), Relaxed);
                go!(1);
            },
            1: if self.push16(bus, VAL16.load(Relaxed)).is_some(){
                go!(2);
            },
            2:{
                go!(0);
                self.fetch(bus);
            }

        })
    }

    pub fn pop16(&mut self, bus: &Peripherals) -> Option<u16> {
        step!(None, {
            0: {
                VAL8.store(bus.read(self.regs.sp), Relaxed);
                self.regs.sp = self.regs.sp.wrapping_add(1);
                go!(1);
                return None;
            },
            1: {
                let hi = bus.read(self.regs.sp);
                self.regs.sp = self.regs.sp.wrapping_add(1);
                VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);
                go!(2);
                return None;
            },
            2: {
                go!(0);
                return Some(VAL16.load(Relaxed));
            }
        })
    }

    pub fn pop(&mut self, bus: &mut Peripherals, dst: Reg16) {
        if let Some(v) = self.pop16(bus) {
            self.write16(bus, dst, v);
            self.fetch(bus);
        }
    }

    pub fn jr(&mut self, bus: &Peripherals) {
        step!((),{
            0: if let Some(v) = self.read8(bus, Imm8){
                self.regs.pc = self.regs.pc.wrapping_add(v as i8 as u16); // 即値を符号付き整数とみなしてPC レジスタに足す
                return go!(1);
            },
            1:{
                go!(0);
                self.fetch(bus);
            }
        })
    }
}

macro_rules! step {
    ($d: expr, {$($c:tt : $e:expr,)*}) => {
        static STEP: AtomicU8 = AtomicU8::new(0);
        #[allow(dead_code)]
        static VAL8: AtomicU8 = AtomicU8::new(0);
        #[allow(dead_code)]
        static VAL16: AtomicU16 = AtomicU16::new(0);
        $(if STEP.load(Relaxed) == $c { $e })* else {return $d;}
    };
}
pub(crate) use step;

macro_rules! go {
    ($e: expr) => {
        STEP.store($e, Relaxed)
    };
}
pub(crate) use go;
