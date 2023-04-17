pub struct ArgsConstructor {
    commands: String,
}

// TODO: Implement flag system
impl ArgsConstructor {
    pub fn new() -> Self {
        let mut args = std::env::args().collect::<Vec<String>>();
        args.remove(0);

        return ArgsConstructor {
            commands: args.into_iter().filter(|e| !e.starts_with("--")).collect::<Vec<String>>().join(""),
        };
    }

    pub fn get_commands(&self) -> Vec<char> {
        return self.commands.chars().collect::<Vec<char>>();
    }
}
