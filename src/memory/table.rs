use std::cell::RefCell;

use prettytable::{Cell, Table, format, Row, row};
use crate::memory::manager::MemoryManager;

pub struct MemoryTable {
    memoman: RefCell<MemoryManager>,

    pub row_titles: RefCell<Row>,
    pub row_memory: RefCell<Row>,
    pub row_swap: RefCell<Row>,

    show_memory: bool,
    show_swap: bool,

    quiet: bool,
}

enum InsertType {
    Title,
    Memory,
    Swap,
}

impl MemoryTable {
    pub fn new(memoman: MemoryManager, show_memory: bool, show_swap: bool, quiet: bool) -> Self {
        let row_titles: Row = if !quiet { row![""] } else { row![] };

        return MemoryTable {
            memoman: RefCell::new(memoman),

            row_titles: RefCell::new(row_titles),
            row_memory: RefCell::new(if quiet { row![] } else { row!["Memory"] }),
            row_swap: RefCell::new(if quiet { row![] } else { row!["Swap"] }),

            show_memory,
            show_swap,

            quiet,
        };
    }

    fn insert_cell(&self, at: InsertType, information: String) {
        let cell: Cell = Cell::from(&information);

        match at {
            InsertType::Title =>
                if !self.quiet || (!self.show_memory && !self.show_swap) {
                    self.row_titles.borrow_mut().add_cell(cell);
                }

            InsertType::Memory =>
                if self.show_memory || (!self.show_memory && !self.show_swap) {
                    self.row_memory.borrow_mut().add_cell(cell);
                },

            InsertType::Swap =>
                if self.show_swap || (!self.show_memory && !self.show_swap) {
                    self.row_swap.borrow_mut().add_cell(cell);
                },
        }
    }

    fn insert_information(&self, title: &str, memory: u64, swap: u64) {
        self.insert_cell(InsertType::Title, String::from(title));
        self.insert_cell(InsertType::Memory, memory.to_string());
        self.insert_cell(InsertType::Swap, swap.to_string());
    }

    pub fn add_total(&mut self) {
        let total = self.memoman.borrow_mut().get_total();

        self.insert_information("Total", total.0, total.1);
    }

    pub fn add_free(&mut self) {
        let free = self.memoman.borrow_mut().get_free();

        self.insert_information("Free", free.0, free.1);
    }

    pub fn add_used(&mut self) {
        let used = self.memoman.borrow_mut().get_used();

        self.insert_information("Used", used.0, used.1);
    }

    pub fn add_memory_available(&mut self) {
        let available = self.memoman.borrow_mut().get_memory_available();

        self.insert_cell(InsertType::Title, String::from("Available"));
        self.insert_cell(InsertType::Memory, available.to_string())
    }

    pub fn is_empty(&self) -> bool {
        // Determine how many elements each vector should have based on the value of self.quiet
        let min_elements: usize = !self.quiet as usize;

        // Check if all of the vectors are empty (quiet == 0) or not (quiet == 1)
        return self.row_titles.borrow().len() == min_elements && self.row_memory.borrow().len() == min_elements && self.row_swap.borrow().len() == min_elements;
    }

    pub fn assemble(&mut self) -> Table {
        let mut table: Table = Table::new();
        table.set_format(format::FormatBuilder::new().padding(0, 5).build());

        if !self.quiet { table.set_titles(self.row_titles.borrow().to_owned()); }

        match (self.show_memory, self.show_swap) {
            (true, false) => {
                table.add_row(self.row_memory.borrow().to_owned());
            },

            (false, true) => {
                table.add_row(self.row_swap.borrow().to_owned());
            },

            _ => {
                table.add_row(self.row_memory.borrow().to_owned());
                table.add_row(self.row_swap.borrow().to_owned());
            },
        };

        return table;
    }
}
