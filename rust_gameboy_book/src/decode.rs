impl Cpu {
    pub fn decode(&mut self, bus: &mut Peripherals) {
        if self.ctx.cb {
            // 16bitのopcode命令を実行中である場合
            self.cb_decode(bus);
            return;
        }

        match self.ctx.opcode {
            0x00 => self.nop(bus),
            0x20 => self.jr_c(bus, Cond::NZ),
            0xCB => self.cb_prefixed(bus), // 16bitのopcode命令のprefix

            _ => panic!("Not implemented: {:02x}", self.opcode),
        }
    }

    pub fn cb_decode(&mut self, bus: &mut Peripherals) {
        match self.ctx.opcode {
            0x10 => self.rl(bus, Reg8::B),

            _ => panic!("Not implemented: {:02x}", self.opcode),
        }
    }

    pub fn cb_prefixed(&mut self, bus: &mut Peripherals) {
        if let Some(v) = self.read8(bus, Imm8) {
            self.ctx.opcode = v;
            self.ctx.cb = true;
            self.cb_decode(bus);
        }
    }
}
