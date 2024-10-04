use std::{env::args, fs::File, io::Read, process::exit};

#[derive(Debug)] // This allows us to print the structure easily with `dbg!`
struct Grammar {
    def: String,
    new: String,
}

fn main() {
    let defengine = gen_grm();
    let mut usrgrm: Vec<Grammar> = Vec::new();
    let args: Vec<String> = args().collect();

    // Combine arguments that start with a dot ('.') with the previous argument
    let processed_args = combine_args(args);

    // Check if the correct number of arguments is provided
    if processed_args.len() < 2 {
        eprintln!("Whoa there, coding wizard! You forgot to tell me which spell to cast! Usage: program_name -i=<input_file> [-u=<user_grammar_file>] [-nf=<neit_file>]");
        exit(1);
    }

    let mut input_file = String::new();
    let mut user_grammar_file = String::new();
    let mut neit_file = String::new();

    // Parse command line arguments
    for arg in &processed_args[1..] {
        if arg.starts_with("-i=") {
            input_file = arg.trim_start_matches("-i=").to_string();
        } else if arg.starts_with("-u=") {
            user_grammar_file = arg.trim_start_matches("-u=").to_string();
        } else if arg.starts_with("-nf=") {
            neit_file = arg.trim_start_matches("-nf=").to_string();
        } else {
            eprintln!("Unknown argument: {}", arg);
            exit(1);
        }
    }

    // Debug the parsed files
    println!("Input file: {}", input_file);
    println!("User grammar file: {}", user_grammar_file);
    println!("Neit file: {}", neit_file);

    // Process the input grammar file
    if !input_file.is_empty() {
        process_grammar_file(&input_file, &mut usrgrm);
    }

    // Process the user grammar file if provided
    if !user_grammar_file.is_empty() {
        process_grammar_file(&user_grammar_file, &mut usrgrm);
    }

    // Process the neit file if provided
    if !neit_file.is_empty() {
        process_neit_file(&neit_file, &usrgrm, &defengine);
    }
}

// Function to combine arguments starting with a dot ('.') with the previous argument
fn combine_args(args: Vec<String>) -> Vec<String> {
    let mut combined_args = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let mut arg = args[i].clone();

        // If the next argument starts with a dot, combine it with the current one
        while i + 1 < args.len() && args[i + 1].starts_with('.') {
            arg.push_str(&args[i + 1]);
            i += 1;
        }

        combined_args.push(arg);
        i += 1;
    }

    combined_args
}

// Function to process grammar files
fn process_grammar_file(file_path: &str, usrgrm: &mut Vec<Grammar>) {
    match File::open(file_path) {
        Ok(mut file) => {
            let mut content = String::new();
            if let Err(e) = file.read_to_string(&mut content) {
                eprintln!(
                    "Error reading the source grammar file '{}': {}",
                    file_path, e
                );
                exit(1);
            }
            let mut index = 1;
            for ln in content.lines() {
                if ln.starts_with("#") {
                    continue; // Skip comments
                } else {
                    let pts: Vec<&str> = ln.split("~").collect();
                    if pts.len() != 2 {
                        eprintln!(
                            "Error on line({}) in the file '{}' : {}",
                            index, file_path, ln
                        );
                        exit(1);
                    }
                    let ogv = pts[0].trim(); // Original value
                    let nv = pts[1].trim(); // New value

                    usrgrm.push(Grammar {
                        def: ogv.to_string(),
                        new: nv.to_string(),
                    });
                }
                index += 1;
            }
        }
        Err(_) => {
            eprintln!(
                "Could not find the grammar file '{}'. Ensure it exists.",
                file_path
            );
            exit(1);
        }
    }
}

// Function to process the neit file
fn process_neit_file(file_path: &str, usrgrm: &[Grammar], defengine: &[Grammar]) {
    match File::open(file_path) {
        Ok(mut file) => {
            let mut content = String::new();
            if let Err(e) = file.read_to_string(&mut content) {
                eprintln!("Error reading file '{}': {}", file_path, e);
                exit(1);
            }
            let mut modified_content = String::new();
            let mut current_word = String::new();
            let mut in_string_mode = false;

            for c in content.chars() {
                if c == '"' {
                    in_string_mode = !in_string_mode;
                    modified_content.push(c);
                    continue;
                }

                if in_string_mode {
                    modified_content.push(c);
                } else {
                    if c.is_whitespace() || c.is_ascii_punctuation() {
                        if !current_word.is_empty() {
                            let replaced_word = replace_word(&current_word, usrgrm, defengine);
                            modified_content.push_str(&replaced_word);
                            current_word.clear();
                        }
                        modified_content.push(c);
                    } else {
                        current_word.push(c);
                    }
                }
            }

            // Append any remaining word after the loop ends
            if !current_word.is_empty() {
                let replaced_word = replace_word(&current_word, usrgrm, defengine);
                modified_content.push_str(&replaced_word);
            }
            println!("{}", modified_content);
        }
        Err(_) => {
            eprintln!("Could not open neit file '{}'", file_path);
            exit(1);
        }
    }
}

// Helper function to replace a word if it matches grammar definitions
fn replace_word(word: &str, usrgrm: &[Grammar], defengine: &[Grammar]) -> String {
    for mapping in usrgrm.iter().chain(defengine.iter()) {
        if word == mapping.new {
            return mapping.def.clone();
        }
    }
    word.to_string()
}

// Function to generate default grammar mappings
fn gen_grm() -> Vec<Grammar> {
    vec![
        Grammar {
            def: "fn".to_string(),
            new: "fn".to_string(),
        },
        Grammar {
            def: "may".to_string(),
            new: "may".to_string(),
        },
        Grammar {
            def: "must".to_string(),
            new: "must".to_string(),
        },
        Grammar {
            def: "=".to_string(),
            new: "=".to_string(),
        },
        Grammar {
            def: "pub".to_string(),
            new: "pub".to_string(),
        },
        Grammar {
            def: "takein".to_string(),
            new: "takein".to_string(),
        },
        Grammar {
            def: "println".to_string(),
            new: "println".to_string(),
        },
        Grammar {
            def: "print".to_string(),
            new: "print".to_string(),
        },
    ]
}
