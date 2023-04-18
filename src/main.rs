mod utils;
mod memory;

use crate::utils::args::ArgsConstructor;
use crate::utils::help::HelpMenu;
use crate::memory::table::MemoryTable;
use crate::memory::manager::MemoryManager;

fn main() {
    let argman = ArgsConstructor::new();

    colored::control::set_override(argman.contains('C'));

    argman.functionize('h', || {
        HelpMenu::new()
            .add_command('t', "Add total memory/swap to the output")
            .add_command('f', "Add free memory/swap to the output")
            .add_command('u', "Add used memory/swap to the output (req testing)")
            .add_command('a', "Add available memory to the output")
            .add_flag('h', "Show help menu")
            .add_flag('m', "Add memory layer to the output")
            .add_flag('s', "Add swap layer to the output")
            .add_flag('b', "Show result in B instead of KB")
            .add_flag('q', "Disable headers and naming, quiet output")
            .add_flag('C', "Enable colored output")
            .assemble();
    });

    let flag_memory: bool = argman.contains('m');
    let flag_swap: bool = argman.contains('s');
    let flag_as_bytes: bool = argman.contains('b');
    let flag_quiet: bool = argman.contains('q');

    let mut memoman: MemoryManager = MemoryManager::new(flag_as_bytes);
    memoman.refresh();

    let mut table = MemoryTable::new(memoman, flag_memory, flag_swap, flag_quiet);

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
