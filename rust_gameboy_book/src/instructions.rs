impl Cpu {
    pub fn nop(&mut self, bus: &Peripherals) {
        self.fetch(bus);
    }
}
