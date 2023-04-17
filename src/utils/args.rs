const FLAG_IDENTIFIER: &str = "-";

pub struct ArgsConstructor {
    pub commands: Vec<char>,
    pub flags: Vec<String>,
}

impl ArgsConstructor {
    pub fn new() -> Self {
        let args: Vec<String> = std::env::args().skip(1).collect::<Vec<String>>();

        let commands = args
            .iter()
            .filter(|e| !e.starts_with(FLAG_IDENTIFIER))
            .map(|e| e.chars())
            .flatten()
            .collect::<Vec<char>>();

        let flags = args
            .iter()
            .filter(|e| e.starts_with(FLAG_IDENTIFIER))
            .cloned()
            .collect::<Vec<String>>();

        return ArgsConstructor {
            commands,
            flags,
        };
    }

    pub fn get_flags(&self) -> Vec<char> {
        return self.flags
            .iter()
            .filter(|f| f.starts_with(FLAG_IDENTIFIER))
            .flat_map(|f| f.chars().skip(1))
            .collect();
    }

    pub fn contains(&self, flag: char) -> bool {
        return self.get_flags().iter().find(|e| **e == flag).is_some();
    }

    pub fn functionize<F>(&self, flag: char, mut callback: F) -> () where F: FnMut() {
        if self.contains(flag) {
            callback();
        }
    }
}
