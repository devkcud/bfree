mod utils;
mod memory;

use utils::argmanager::Argumenter;
use utils::help;
use memory::table;
use memory::manager::MemoryManager;

fn main() {
    let arguer = Argumenter::new("-");

    if arguer.has_flag(vec!["help", "h"]).exists() {
        help::HelpMenu::new(); // TODO: Create help menu
        return;
    }

    let flag_memory: bool = arguer.has_flag(vec!["memory", "memo", "m"]).exists();
    let flag_swap: bool = arguer.has_flag(vec!["swap", "s"]).exists();
    let flag_as_bytes: bool = arguer.has_flag(vec!["bytes", "b"]).exists();
    let flag_quiet: bool = arguer.has_flag(vec!["quiet", "q"]).exists();

    let mut memoman: MemoryManager = MemoryManager::new(flag_as_bytes);
    memoman.refresh();

    let mut table = table::MemoryTable::new(memoman, flag_memory, flag_swap, flag_quiet);

    for c in arguer.commands.join(" ").trim().chars() {
        match c {
            't' => table.add_total(),
            'f' => table.add_free(),
            'u' => table.add_used(),
            'a' => table.add_memory_available(),
            _ => {},
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
