mod highlight_in_line;
mod upload_config;

use upload_config::upload_config;

use highlight_in_line::highlight_in_line;

use std::{
    env,
    io::{stdin, Read},
    process,
};

struct SearchOptions {
    sensitive_register: bool,
    show_line_index: bool,
}

fn main() {
    let config = upload_config();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Not found search value");
        process::exit(1);
    }

    let searching_value = args.get(args.len() - 1).unwrap_or_else(|| {
        println!("Searching value not found");
        println!("Example: cat file.txt | minigrep 'some value'");
        process::exit(1);
    });

    let mut content: String = String::new();

    stdin().read_to_string(&mut content).unwrap_or_else(|_| {
        println!("Failed to read data");
        process::exit(1);
    });

    let search_options = SearchOptions {
        sensitive_register: args.contains(&"-r".to_string()),
        show_line_index: args.contains(&"-i".to_string()),
    };

    let mut not_found: bool = true;

    for (index, line) in content.lines().enumerate() {
        let inner_line = if search_options.sensitive_register {
            line
        } else {
            &line.to_lowercase()
        };

        let inner_searching_value = if search_options.sensitive_register {
            searching_value
        } else {
            &searching_value.to_lowercase()
        };

        if inner_line.contains(inner_searching_value) {
            let highlight_value =
                highlight_in_line(line.to_string(), searching_value.to_string(), &config);

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
        println!("Not found '{}'", searching_value)
    }
}
