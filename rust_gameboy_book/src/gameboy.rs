use sdl2;

impl GameBoy {
    pub fn new(bootrom: Bootrom) -> self {
        let sdl = sdl2::init().expect("failed to initialize SDL");
        let lcd = LCD::new(&sdl, 4);
        let peripherals = Peripherals::new(bootrom);
        let cpu = Cpu::new();
        Self {
            cpu,
            peripherals,
            lcd,
        }
    }

    pub fn ru(&mut self) {
        let time = time::Instant::now();
        let mut elapsed = 0;

        loop {
            let e = time.elapsed().as_nanos();
            for _ in 0..(e - elapsed) / M_CYCLE_NANOS {
                self.cpu.emulate_cycle(&mut self.peripherals);
                if self.peripherals.ppu.emulate_cycle() {
                    self.lcd.draw(self.peripherals.ppu.pixcel_buffer());
                }

                elapsed += M_CYCLE_NANOS;
            }
        }
    }
}
