const FLAG_IDENTIFIER_LONG: &str = "--";
const FLAG_IDENTIFIER_SHORT: &str = "-";

pub struct ArgsConstructor {
    pub commands: Vec<char>,
    flags: Vec<String>,
}

impl ArgsConstructor {
    pub fn new() -> Self {
        let args: Vec<String> = std::env::args().skip(1).collect::<Vec<String>>();

        let commands = args
            .iter()
            .filter(|e| !e.starts_with(FLAG_IDENTIFIER_SHORT))
            .map(|e| e.chars())
            .flatten()
            .collect::<Vec<char>>();

        let flags = args
            .iter()
            .filter(|e| e.starts_with(FLAG_IDENTIFIER_LONG) || e.starts_with(FLAG_IDENTIFIER_SHORT))
            .cloned()
            .collect::<Vec<String>>();

        return ArgsConstructor {
            commands,
            flags,
        };
    }

    pub fn get_flags(&self) -> (Vec<String>, Vec<char>) {
        let flags: &Vec<String> = &self.flags;

        let long_flags: Vec<String> = flags.iter().filter(|f| f.starts_with(FLAG_IDENTIFIER_LONG)).map(|f| f.trim_start_matches(FLAG_IDENTIFIER_LONG).to_string()).collect();
        let short_flags: Vec<char> = flags.iter().filter(|f| f.starts_with(FLAG_IDENTIFIER_SHORT)).flat_map(|f| f.chars().skip(1)).collect();

        return (long_flags, short_flags);
    }

    pub fn exists(&self, long_flag: &str, short_flag: &str) -> bool {
        return self.get_flags().0.iter().find(|e| *e == long_flag).is_some() || self.get_flags().1.iter().find(|e| e.to_string() == short_flag).is_some();
    }

    pub fn functionize<F>(&self, long_flag: &str, short_flag: &str, mut callback: F) -> () where F: FnMut() {
        self.exists(long_flag, short_flag).then(|| callback());
    }
}
