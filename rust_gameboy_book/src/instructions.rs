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
