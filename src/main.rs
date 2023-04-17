mod utils;
mod memory;

use crate::utils::args::ArgsConstructor;
use crate::memory::table;
use crate::memory::manager::MemoryManager;

fn main() {
    let argman = ArgsConstructor::new();

    // TODO: Create help menu
    argman.functionize('h', || todo!("Help menu"));

    let flag_memory: bool = argman.contains('m');
    let flag_swap: bool = argman.contains('s');
    let flag_as_bytes: bool = argman.contains('b');
    let flag_quiet: bool = argman.contains('q');

    let mut memoman: MemoryManager = MemoryManager::new(flag_as_bytes);
    memoman.refresh();

    let mut table = table::MemoryTable::new(memoman, flag_memory, flag_swap, flag_quiet);

    for command in argman.commands {
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
