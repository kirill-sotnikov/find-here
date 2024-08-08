fn get_argument_value(args: &Vec<String>, arg: &str) -> Option<String> {
    let mut catch_next_value = false;
    for item in args {
        if catch_next_value {
            return Some(item.to_string());
        }

        if item == arg {
            catch_next_value = true
        }
    }

    None
}

pub struct SearchOptions {
    pub sensitive_register: bool,
    pub show_line_index: bool,
    pub file: Option<String>,
}

impl SearchOptions {
    pub fn from(args: &Vec<String>) -> Self {
        SearchOptions {
            sensitive_register: args.contains(&"-r".to_string()),
            show_line_index: args.contains(&"-i".to_string()),
            file: get_argument_value(args, "-f"),
        }
    }
}
