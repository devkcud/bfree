use sysinfo::{System, SystemExt};

const KB_SIZE: u64 = 2_u64.overflowing_pow(20).0;

pub struct MemoryManager {
    system: System,
    as_bytes: bool,
}

impl MemoryManager {
    pub fn new(as_bytes: bool) -> Self {
        return MemoryManager {
            system: System::new(),
            as_bytes,
        };
    }

    pub fn refresh(&mut self) -> () {
        return self.system.refresh_memory();
    }

    pub fn get_total(&mut self) -> (u64, u64) {
        let memory: u64 = self.system.total_memory();
        let swap: u64 = self.system.total_swap();

        return match self.as_bytes {
            true => (memory, swap),
            false => (memory.overflowing_div(KB_SIZE).0, swap.overflowing_div(KB_SIZE).0),
        };
    }

    pub fn get_free(&mut self) -> (u64, u64) {
        let memory: u64 = self.system.free_memory();
        let swap: u64 = self.system.free_swap();

        return match self.as_bytes {
            true => (memory, swap),
            false => (memory.overflowing_div(KB_SIZE).0, swap.overflowing_div(KB_SIZE).0),
        };
    }

    pub fn get_used(&mut self) -> (u64, u64) {
        let memory: u64 = self.system.used_memory();
        let swap: u64 = self.system.used_swap();

        return match self.as_bytes {
            true => (memory, swap),
            false => (memory.overflowing_div(KB_SIZE).0, swap.overflowing_div(KB_SIZE).0),
        };
    }

    pub fn get_memory_available(&mut self) -> u64 {
        let memory: u64 = self.system.available_memory();

        return match self.as_bytes {
            true => memory,
            false => memory.overflowing_div(KB_SIZE).0,
        }
    }
}
