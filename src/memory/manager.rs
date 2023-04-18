use sysinfo::{System, SystemExt};

const KB_SIZE: u64 = 1000 << 10;

pub struct MemoryManager {
    system: System,
    as_bytes: bool,
    factor: u64,
}

impl MemoryManager {
    pub fn new(as_bytes: bool) -> Self {
        return MemoryManager {
            system: System::new(),
            as_bytes,
            factor: if as_bytes { 1 } else { KB_SIZE },
        };
    }

    pub fn refresh(&mut self) -> () {
        self.system.refresh_memory();
        return;
    }

    pub fn get_total(&mut self) -> (u64, u64) {
        let memory = self.system.total_memory();
        let swap = self.system.total_swap();

        return (memory / self.factor, swap / self.factor);
    }

    pub fn get_free(&mut self) -> (u64, u64) {
        let memory = self.system.free_memory();
        let swap = self.system.free_swap();

        return (memory / self.factor, swap / self.factor);
    }

    pub fn get_used(&mut self) -> (u64, u64) {
        let memory = self.system.used_memory();
        let swap = self.system.used_swap();

        return (memory / self.factor, swap / self.factor);
    }

    pub fn get_memory_available(&mut self) -> u64 {
        let memory = self.system.available_memory();

        return memory / self.factor;
    }
}
