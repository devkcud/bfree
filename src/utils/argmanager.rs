fn ele_split(element: String, split: char, index: i8, default: &str) -> String {
    String::from(element.split(split).nth(index as usize).unwrap_or(default))
}

pub struct Argumenter {
    pub commands: Vec<String>,
    pub flags: Vec<String>,
}

impl Argumenter {
    pub fn new(flag_definer: &str) -> Argumenter {
        let mut args = std::env::args().collect::<Vec<String>>();
        args.remove(0); // Remove the executable as the argument

        let commands: Vec<String> = args.iter().filter(|e| !e.starts_with(flag_definer)).map(|e| e.to_string()).collect();
        let flags: Vec<String> = args.iter().filter(|e| e.starts_with(flag_definer)).map(|e| e.to_string().replace(flag_definer, "")).collect();

        return Argumenter {
            commands,
            flags,
        };
    }

    pub fn has_flag(&self, flag_names: Vec<&str>) -> Flag {
        return Flag {
            content: String::from(self.flags.iter().find(|e| {
                return flag_names.contains(&&ele_split(e.to_string(), '=', 0, "").as_str());
            }).unwrap_or(&String::new()))
        };
    }
}

pub struct Flag {
    content: String,
}

impl Flag {
    pub fn exists(&self) -> bool {
        return self.content != "";
    }

    pub fn get_argument(&self) -> String {
        ele_split(self.content.to_string(), '=', 1, "")
    }
}
