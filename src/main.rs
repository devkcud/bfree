mod utils;
mod memory;

use crate::utils::args::ArgsConstructor;
use crate::memory::table;
use crate::memory::manager::MemoryManager;

fn main() {
    let argman = ArgsConstructor::new();

    // TODO: Create help menu
    argman.functionize("help", "h", || todo!("Help menu"));

    let flag_memory: bool = argman.exists("memory", "m");
    let flag_swap: bool = argman.exists("swap", "s");
    let flag_as_bytes: bool = argman.exists("byte", "b");
    let flag_quiet: bool = argman.exists("quiet", "q");

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
