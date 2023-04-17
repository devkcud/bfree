use std::cell::RefCell;

const FLAG_IDENTIFIER_LONG: &str = "--";
const FLAG_IDENTIFIER_SHORT: &str = "-";

pub struct ArgsConstructor {
    commands: Vec<String>,
    flags: Vec<String>,
}

impl ArgsConstructor {
    pub fn new() -> Self {
        let args = RefCell::new(std::env::args().collect::<Vec<String>>());
        args.borrow_mut().remove(0);
        let args = args.borrow();

        fn is_flag(e: &str) -> bool {
            return e.starts_with(FLAG_IDENTIFIER_LONG) || e.starts_with(FLAG_IDENTIFIER_SHORT);
        }

        let commands = args.iter().filter(|e| !is_flag(e));
        let flags = args.iter().filter(|e| is_flag(e));

        return ArgsConstructor {
            commands: commands.map(|e| e.to_string()).collect::<Vec<String>>(),
            flags: flags.map(|e| e.to_string()).collect::<Vec<String>>(),
        };
    }

    pub fn get_commands(&self) -> Vec<char> {
        let mut cmds = self.commands.join("").chars().collect::<Vec<char>>();
        cmds.dedup();

        return cmds;
    }

    pub fn get_flags(&self) -> (Vec<String>, Vec<char>) {
        let flags = &self.flags;

        let long_flags: Vec<String> = flags.iter().filter(|e| e.starts_with(FLAG_IDENTIFIER_LONG)).map(|e| e.replace(FLAG_IDENTIFIER_LONG, "")).collect::<Vec<String>>();
        let short_flags: Vec<String> = flags.iter().filter(|e| !e.starts_with(FLAG_IDENTIFIER_LONG) && e.starts_with(FLAG_IDENTIFIER_SHORT)).map(|e| e.to_string()).collect::<Vec<String>>();

        let short_flags: Vec<char> = short_flags.join("").replace(FLAG_IDENTIFIER_SHORT, "").chars().collect::<Vec<char>>();

        return (long_flags, short_flags);
    }

    pub fn exists(&self, long_flag: &str, short_flag: &str) -> bool {
        return self.get_flags().0.iter().find(|e| *e == long_flag).is_some() || self.get_flags().1.iter().find(|e| e.to_string() == short_flag).is_some();
    }

    pub fn functionize<F>(&self, long_flag: &str, short_flag: &str, mut callback: F) -> () where F: FnMut() {
        if self.exists(long_flag, short_flag) { callback(); }
    }
}
