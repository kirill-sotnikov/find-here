use colored::{ColoredString, Colorize};

use crate::upload_config::{self, Config};

fn highlight(value: &String, config: &Config) -> ColoredString {
    let value_clone = value.clone();
    let mut result: ColoredString;

    match config.color {
        upload_config::ConfigColor::Green => result = value_clone.bright_green(),
        upload_config::ConfigColor::Blue => result = value_clone.blue(),
        upload_config::ConfigColor::Red => result = value.red(),
        upload_config::ConfigColor::None => result = value_clone.white(),
    }

    if config.background {
        result = result.reversed();
    }

    if config.underline {
        result = result.underline();
    }

    return result;
}

pub fn highlight_in_line(line: String, substring: String, config: &Config) -> String {
    let mut coincidence: String = String::new();
    let mut result = String::new();

    let line_chars: Vec<char> = line.chars().collect();

    let mut substring_char_index: usize = 0;
    let substring_chars: Vec<char> = substring.chars().collect();

    for value in line_chars.iter() {
        let substring_char = substring_chars.get(substring_char_index);

        match substring_char {
            None => {
                substring_char_index = 0;
                coincidence.clear();
                result.push(value.clone());
                continue;
            }
            Some(substring_char) => {
                if *substring_char.to_lowercase().to_string() == *value.to_lowercase().to_string() {
                    coincidence.push(value.clone());
                    substring_char_index += 1;

                    if substring_char_index == substring.len() {
                        result.push_str(&format!("{}", highlight(&coincidence, config)));
                        substring_char_index = 0;
                        coincidence.clear();
                    }
                } else {
                    substring_char_index = 0;
                    result.push_str(&format!("{}{}", coincidence, value));
                    coincidence.clear();
                }
            }
        }
    }

    return result;
}
