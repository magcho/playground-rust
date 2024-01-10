impl Cpu {
    pub fn decode(&mut self, bus: &mut Peripherals) {
        if self.ctx.cb {
            // 16bitのopcode命令を実行中である場合
            self.cb_decode(bus);
            return;
        }

        match self.ctx.opcode {
            0x00 => self.nop(bus),
            0x01 => self.ld16(bus, Reg16::BC, Imm16),
            0x11 => self.ld16(bus, Reg16::DE, Imm16),
            0x21 => self.ld16(bus, Reg16::HL, Imm16),
            0x31 => self.ld16(bus, Reg16::SP, Imm16),

            0x02 => self.ld(bus, Indirect::BC, Imm8),
            0x12 => self.ld(bus, Indirect::DE, Imm8),
            0x22 => self.ld(bus, Indirect::HLI, Imm8),
            0x32 => self.ld(bus, Indirect::HLD, Imm8),

            0x06 => self.ld(bus, Reg8::B, Imm8),
            0x16 => self.ld(bus, Reg8::D, Imm8),
            0x26 => self.ld(bus, Reg8::H, Imm8),
            0x36 => self.ld(bus, Indirect::HL, Imm8),

            0x0A => self.ld(bus, Reg8::A, Indirect::BC),
            0x1A => self.ld(bus, Reg8::A, Indirect::DE),
            0x2A => self.ld(bus, Reg8::A, Indirect::HLI),
            0x3A => self.ld(bus, Reg8::A, Indirect::HLD),

            0x0E => self.ld(bus, Reg8::C, Imm8),
            0x1E => self.ld(bus, Reg8::E, Imm8),
            0x2E => self.ld(bus, Reg8::L, Imm8),
            0x3E => self.ld(bus, Reg8::A, Imm8),

            0x40 => self.ld(bus, Reg8::B, Reg8::B),
            0x50 => self.ld(bus, Reg8::D, Reg8::B),
            0x60 => self.ld(bus, Reg8::H, Reg8::B),
            0x70 => self.ld(bus, Indirect::HL, Reg8::B),

            0x41 => self.ld(bus, Reg8::B, Reg8::C),
            0x51 => self.ld(bus, Reg8::D, Reg8::C),
            0x61 => self.ld(bus, Reg8::H, Reg8::C),
            0x71 => self.ld(bus, Indirect::HL, Reg8::C),

            0x42 => self.ld(bus, Reg8::B, Reg8::D),
            0x52 => self.ld(bus, Reg8::D, Reg8::D),
            0x62 => self.ld(bus, Reg8::H, Reg8::D),
            0x72 => self.ld(bus, Indirect::HL, Reg8::D),

            0x43 => self.ld(bus, Reg8::B, Reg8::E),
            0x53 => self.ld(bus, Reg8::D, Reg8::E),
            0x63 => self.ld(bus, Reg8::H, Reg8::E),
            0x73 => self.ld(bus, Indirect::HL, Reg8::E),

            0x44 => self.ld(bus, Reg8::B, Reg8::H),
            0x54 => self.ld(bus, Reg8::D, Reg8::H),
            0x64 => self.ld(bus, Reg8::H, Reg8::H),
            0x74 => self.ld(bus, Indirect::HL, Reg8::H),

            0x45 => self.ld(bus, Reg8::B, Reg8::L),
            0x55 => self.ld(bus, Reg8::D, Reg8::L),
            0x65 => self.ld(bus, Reg8::H, Reg8::L),
            0x75 => self.ld(bus, Indirect::HL, Reg8::L),

            0x46 => self.ld(bus, Reg8::B, Indirect::HL),
            0x56 => self.ld(bus, Reg8::D, Indirect::HL),
            0x66 => self.ld(bus, Reg8::H, Indirect::HL),

            0x47 => self.ld(bus, Reg8::B, Reg8::A),
            0x57 => self.ld(bus, Reg8::D, Reg8::A),
            0x67 => self.ld(bus, Reg8::H, Reg8::A),
            0x77 => self.ld(bus, Indirect::HL, Reg8::A),

            0x48 => self.ld(bus, Reg8::C, Reg8::B),
            0x58 => self.ld(bus, Reg8::E, Reg8::B),
            0x68 => self.ld(bus, Reg8::L, Reg8::B),
            0x78 => self.ld(bus, Reg8::A, Reg8::B),

            0x49 => self.ld(bus, Reg8::C, Reg8::C),
            0x59 => self.ld(bus, Reg8::E, Reg8::C),
            0x69 => self.ld(bus, Reg8::L, Reg8::C),
            0x79 => self.ld(bus, Reg8::A, Reg8::C),

            0x4A => self.ld(bus, Reg8::C, Reg8::D),
            0x5A => self.ld(bus, Reg8::E, Reg8::D),
            0x6A => self.ld(bus, Reg8::L, Reg8::D),
            0x7A => self.ld(bus, Reg8::A, Reg8::D),

            0x4B => self.ld(bus, Reg8::C, Reg8::E),
            0x5B => self.ld(bus, Reg8::E, Reg8::E),
            0x6B => self.ld(bus, Reg8::L, Reg8::E),
            0x7B => self.ld(bus, Reg8::A, Reg8::E),

            0x4C => self.ld(bus, Reg8::C, Reg8::H),
            0x5C => self.ld(bus, Reg8::E, Reg8::H),
            0x6C => self.ld(bus, Reg8::L, Reg8::H),
            0x7C => self.ld(bus, Reg8::A, Reg8::H),

            0x4D => self.ld(bus, Reg8::C, Reg8::L),
            0x5D => self.ld(bus, Reg8::E, Reg8::L),
            0x6D => self.ld(bus, Reg8::L, Reg8::L),
            0x7D => self.ld(bus, Reg8::A, Reg8::L),

            0x4E => self.ld(bus, Reg8::C, Indirect::HL),
            0x5E => self.ld(bus, Reg8::E, Indirect::HL),
            0x6E => self.ld(bus, Reg8::L, Indirect::HL),
            0x7E => self.ld(bus, Reg8::A, Indirect::HL),

            0x4F => self.ld(bus, Reg8::C, Reg8::A),
            0x5F => self.ld(bus, Reg8::E, Reg8::A),
            0x6F => self.ld(bus, Reg8::L, Reg8::A),
            0x7F => self.ld(bus, Reg8::A, Reg8::A),

            0xE0 => self.ld(bus, Direct8::DFF, Reg8::A),
            0xF0 => self.ld(bus, Reg8::A, Direct8::DFF),

            0xE2 => self.ld(bus, Indirect::CFF, Reg8::A),
            0xF2 => self.ld(bus, Reg8::A, Indirect::CFF),

            0xEA => self.ld(bus, Direct8::D, Reg8::A),
            0xFA => self.ld(bus, Reg8::A, Direct8::D),

            0xB8 => self.cp(bus, Reg8::B),
            0xB9 => self.cp(bus, Reg8::C),
            0xBA => self.cp(bus, Reg8::D),
            0xBB => self.cp(bus, Reg8::E),
            0xBC => self.cp(bus, Reg8::H),
            0xBD => self.cp(bus, Reg8::L),
            0xBE => self.cp(bus, Indirect::HL),
            0xBF => self.cp(bus, Reg8::A),

            0xFE => self.cp(bus, Imm8),

            0x03 => self.inc16(bus, Reg16::BC),
            0x13 => self.inc16(bus, Reg16::DE),
            0x23 => self.inc16(bus, Reg16::HL),
            0x33 => self.inc16(bus, Reg16::SP),

            0x04 => self.inc(bus, Reg8::B),
            0x14 => self.inc(bus, Reg8::D),
            0x24 => self.inc(bus, Reg8::H),
            0x34 => self.inc(bus, Indirect::HL),

            0x0C => self.inc(bus, Reg8::C),
            0x1C => self.inc(bus, Reg8::E),
            0x2C => self.inc(bus, Reg8::L),
            0x3C => self.inc(bus, Reg8::A),

            0x05 => self.dec(bus, Reg8::B),
            0x15 => self.dec(bus, Reg8::D),
            0x25 => self.dec(bus, Reg8::H),
            0x35 => self.dec(bus, Indirect::HL),

            0x0B => self.dec16(bus, Reg16::BC),
            0x1B => self.dec16(bus, Reg16::DE),
            0x2B => self.dec16(bus, Reg16::HL),
            0x3B => self.dec16(bus, Reg16::SP),

            0x0D => self.dec(bus, Reg8::C),
            0x1D => self.dec(bus, Reg8::E),
            0x2D => self.dec(bus, Reg8::L),
            0x3D => self.dec(bus, Reg8::A),

            0xC5 => self.push(bus, Reg16::BC),
            0xD5 => self.push(bus, Reg16::DE),
            0xE5 => self.push(bus, Reg16::HL),
            0xF5 => self.push(bus, Reg16::AF),

            0xC1 => self.pop(bus, Reg16::BC),
            0xD1 => self.pop(bus, Reg16::DE),
            0xE1 => self.pop(bus, Reg16::HL),
            0xF1 => self.pop(bus, Reg16::AF),

            0x18 => self.jr(bus),
            0x28 => self.jr_c(bus, Cond::Z),
            0x38 => self.jr_c(bus, Cond::C),

            0x20 => self.jr_c(bus, Cond::NZ),
            0x30 => self.jr_c(bus, Cond::NC),

            0xCD => self.call(bus),

            0xC9 => self.ret(bus),

            0xCB => self.cb_prefixed(bus), // 16bitのopcode命令のprefix

            _ => panic!("Not implemented: {:02x}", self.opcode),
        }
    }

    pub fn cb_decode(&mut self, bus: &mut Peripherals) {
        match self.ctx.opcode {
            0x10 => self.rl(bus, Reg8::B),
            0x11 => self.rl(bus, Reg8::C),
            0x12 => self.rl(bus, Reg8::D),
            0x13 => self.rl(bus, Reg8::E),
            0x14 => self.rl(bus, Reg8::H),
            0x15 => self.rl(bus, Reg8::L),
            0x16 => self.rl(bus, Indirect::HL),
            0x17 => self.rl(bus, Reg8::A),

            0x40 => self.bit(bus, 0, Reg8::B),
            0x50 => self.bit(bus, 2, Reg8::B),
            0x60 => self.bit(bus, 4, Reg8::B),
            0x70 => self.bit(bus, 6, Reg8::B),

            0x41 => self.bit(bus, 0, Reg8::C),
            0x51 => self.bit(bus, 2, Reg8::C),
            0x61 => self.bit(bus, 4, Reg8::C),
            0x71 => self.bit(bus, 6, Reg8::C),

            0x42 => self.bit(bus, 0, Reg8::D),
            0x52 => self.bit(bus, 2, Reg8::D),
            0x62 => self.bit(bus, 4, Reg8::D),
            0x72 => self.bit(bus, 6, Reg8::D),

            0x43 => self.bit(bus, 0, Reg8::E),
            0x53 => self.bit(bus, 2, Reg8::E),
            0x63 => self.bit(bus, 4, Reg8::E),
            0x73 => self.bit(bus, 6, Reg8::E),

            0x44 => self.bit(bus, 0, Reg8::H),
            0x54 => self.bit(bus, 2, Reg8::H),
            0x64 => self.bit(bus, 4, Reg8::H),
            0x74 => self.bit(bus, 6, Reg8::H),

            0x45 => self.bit(bus, 0, Reg8::L),
            0x55 => self.bit(bus, 2, Reg8::L),
            0x65 => self.bit(bus, 4, Reg8::L),
            0x75 => self.bit(bus, 6, Reg8::L),

            0x46 => self.bit(bus, 0, Indirect::HL),
            0x56 => self.bit(bus, 2, Indirect::HL),
            0x66 => self.bit(bus, 4, Indirect::HL),
            0x76 => self.bit(bus, 6, Indirect::HL),

            0x47 => self.bit(bus, 0, Reg8::A),
            0x57 => self.bit(bus, 2, Reg8::A),
            0x67 => self.bit(bus, 4, Reg8::A),
            0x77 => self.bit(bus, 6, Reg8::A),

            0x48 => self.bit(bus, 1, Reg8::B),
            0x58 => self.bit(bus, 3, Reg8::B),
            0x68 => self.bit(bus, 5, Reg8::B),
            0x78 => self.bit(bus, 7, Reg8::B),

            0x49 => self.bit(bus, 1, Reg8::C),
            0x59 => self.bit(bus, 3, Reg8::C),
            0x69 => self.bit(bus, 5, Reg8::C),
            0x79 => self.bit(bus, 7, Reg8::C),

            0x4A => self.bit(bus, 1, Reg8::D),
            0x5A => self.bit(bus, 3, Reg8::D),
            0x6A => self.bit(bus, 5, Reg8::D),
            0x7A => self.bit(bus, 7, Reg8::D),

            0x4B => self.bit(bus, 1, Reg8::E),
            0x5B => self.bit(bus, 3, Reg8::E),
            0x6B => self.bit(bus, 5, Reg8::E),
            0x7B => self.bit(bus, 7, Reg8::E),

            0x4C => self.bit(bus, 1, Reg8::H),
            0x5C => self.bit(bus, 3, Reg8::H),
            0x6C => self.bit(bus, 5, Reg8::H),
            0x7C => self.bit(bus, 7, Reg8::H),

            0x4D => self.bit(bus, 1, Reg8::L),
            0x5D => self.bit(bus, 3, Reg8::L),
            0x6D => self.bit(bus, 5, Reg8::L),
            0x7D => self.bit(bus, 7, Reg8::L),

            0x4E => self.bit(bus, 1, Indirect::HL),
            0x5E => self.bit(bus, 3, Indirect::HL),
            0x6E => self.bit(bus, 5, Indirect::HL),
            0x7E => self.bit(bus, 7, Indirect::HL),

            0x4F => self.bit(bus, 1, Reg8::A),
            0x5F => self.bit(bus, 3, Reg8::A),
            0x6F => self.bit(bus, 5, Reg8::A),
            0x7F => self.bit(bus, 7, Reg8::A),

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
