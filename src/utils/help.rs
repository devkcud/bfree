use std::{env::args, process::exit};
use prettytable::{Table, format, row};
use colored::Colorize;

pub struct HelpMenu {
    project_name: String,
    project_description: String,

    commands: Vec<(char, String)>,
    flags: Vec<(char, String)>,
}

impl HelpMenu {
    pub fn new(project_description: &str) -> Self {
        return HelpMenu {
            // May God forgive my sins.
            project_name: String::from(*args().collect::<Vec<String>>().first().unwrap_or(&"bfree".to_string()).to_string().split('/').collect::<Vec<&str>>().last().unwrap_or(&"bfree")),
            project_description: String::from(project_description),

            commands: Vec::new(),
            flags: Vec::new(),
        };
    }

    pub fn add_command(&mut self, id: char, description: &str) -> &mut Self {
        self.commands.push((id, String::from(description)));
        return self;
    }

    pub fn add_flag(&mut self, id: char, description: &str) -> &mut Self {
        self.flags.push((id, String::from(description)));
        return self;
    }

    pub fn assemble(&self) {
        let mut command_table = Table::new();
        command_table.set_format(*format::consts::FORMAT_BOX_CHARS);
        command_table.set_titles(row!["Command".yellow().bold(), "Description".yellow().bold()]);

        let mut flag_table = Table::new();
        flag_table.set_format(*format::consts::FORMAT_BOX_CHARS);
        flag_table.set_titles(row!["Flag   ".yellow().bold(), "Description".yellow().bold()]);

        for (command, description) in &self.commands {
            command_table.add_row(row![command.to_string().green(), description]);
        }

        for (flag, description) in &self.flags {
            let mut flag = flag.to_string();
            flag.insert(0, '-');

            flag_table.add_row(row![flag.to_string().green(), description]);
        }

        command_table.printstd();
        flag_table.printstd();

        println!("\nE.g.:\n\t{} {}", "bfree -mq tff".bold(), "# This will show the memory total and free (2 times) output without headers".dimmed().italic());

        exit(0);
    }
}
