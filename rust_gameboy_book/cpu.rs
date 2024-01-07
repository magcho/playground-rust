#[derive(Default)]
struct Ctx {
    opcode: u8,
    cd: bool,
}

pub struct Cpu {
    regs: Registers,
    ctx: Ctx,
}
