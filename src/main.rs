mod highlight_in_line;
mod search_options;
mod upload_config;

use search_options::SearchOptions;
use upload_config::{upload_config, Config};

use highlight_in_line::highlight_in_line;

use std::{
    env,
    fs::File,
    io::{stdin, Read},
    process,
};

fn read_file(content: &mut String, file_path: &String) -> Result<(), String> {
    match File::open(file_path) {
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => return Err(format!("File {} not found", file_path)),
            std::io::ErrorKind::PermissionDenied => {
                return Err(format!("Read file: permission denied"))
            }
            _ => return Err(format!("Failed to read file {}", file_path)),
        },
        Ok(mut file) => match file.read_to_string(content) {
            Err(error) => Err(error.to_string()),
            _ => Ok(()),
        },
    }
}

fn main() {
    let config = upload_config();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Not found search value");
        process::exit(1);
    }

    let search_value = args.get(args.len() - 1).unwrap_or_else(|| {
        println!("Search value not found");
        println!("Example: cat file.txt | minigrep 'search value'");
        process::exit(1);
    });

    let search_options = SearchOptions::from(&args);

    let mut content: String = String::new();

    match &search_options.file {
        None => {
            stdin().read_to_string(&mut content).unwrap_or_else(|_| {
                println!("Failed to read data");
                process::exit(1);
            });
        }
        Some(file_path) => match read_file(&mut content, file_path) {
            Err(error) => {
                println!("{}", error);
                process::exit(1);
            }
            _ => (),
        },
    }

    run(&content, search_value, &search_options, &config);
}

fn run(content: &str, search_value: &String, search_options: &SearchOptions, config: &Config) {
    let mut not_found: bool = true;

    for (index, line) in content.lines().enumerate() {
        let inner_line = if search_options.sensitive_register {
            line
        } else {
            &line.to_lowercase()
        };

        let inner_searching_value = if search_options.sensitive_register {
            search_value
        } else {
            &search_value.to_lowercase()
        };

        if inner_line.contains(inner_searching_value) {
            let highlight_value =
                highlight_in_line(line.to_string(), search_value.to_string(), &config);

            if search_options.show_line_index == true {
                let line_index = index + 1;
                println!("{}: {}", line_index, highlight_value);
            } else {
                println!("{highlight_value}");
            }
            not_found = false;
        }
    }

    if not_found == true {
        println!("Not found '{}'", search_value)
    }
}
