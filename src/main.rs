mod utils;
mod memory;

use crate::utils::args::ArgsConstructor;
use crate::memory::table;
use crate::memory::manager::MemoryManager;

fn main() {
    let argman = ArgsConstructor::new();

    // TODO: Create a memory flag, show only memory
    let flag_memory: bool = true;
    // TODO: Create a swap flag, show only swap
    let flag_swap: bool = true;
    // TODO: Create a as_bytes flag, show result as bytes
    let flag_as_bytes: bool = false;
    // TODO: Create a quiet flag, don't show extra output
    let flag_quiet: bool = false;

    let mut memoman: MemoryManager = MemoryManager::new(flag_as_bytes);
    memoman.refresh();

    let mut table = table::MemoryTable::new(memoman, flag_memory, flag_swap, flag_quiet);

    for command in argman.get_commands() {
        match command {
            't' => table.add_total(),
            'f' => table.add_free(),
            'u' => table.add_used(),
            'a' => table.add_memory_available(),
            _ => (),
        }
    }

    if table.is_empty() {
        table.add_total();
        table.add_free();
        table.add_used();
        table.add_memory_available();
    }

    table.assemble().printstd();
}
