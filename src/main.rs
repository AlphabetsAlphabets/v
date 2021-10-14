use std::process::Command;
use std::env::args;
use std::path::Path;

const HELP_MESSAGE: &str = r#"Usage: v [options]
Options:
1. -t, --target [dir] search for all files in dir, if empty defaults to $HOME
2. -e, --editor [editor] opens the searched file with [editor], if empty defaults to $EDITOR
3. -h, --ignore-hidden [true/false] if false then does not ignore hidden directories, or files. Defaults to true.

Examples:
1. v -t $HOME/Documents -e nano -h false
Tells v to search for files in $HOME/Documents and to show hidden files, and directories. And to open the file in nano.

2. v -e nano
Tells v to search for files in $HOME and to open the file in nano.
"#;

#[derive(Debug)]
struct Config {
    target_dir: String,
    editor: String,
    ignore_hidden: bool
}

impl Default for Config {
    fn default() -> Self {
        Self {
            target_dir: env!("HOME").to_string(),
            editor: env!("EDITOR").to_string(),
            ignore_hidden: true,
        }
    }
}

fn parse_args(mut args: Vec<String>) -> Config {
    let mut config = Config::default();
    let mut args = args.iter();


    loop {
        if let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--help" => {
                    eprintln!("{}", HELP_MESSAGE);
                    std::process::exit(2);
                }

                "-t" | "--target" => {
                    let t = args.next().unwrap();
                    if !t.is_empty() {
                        let dir = Path::new(t);
                        if dir.is_dir() && dir.exists() {
                            config.target_dir = t.to_string();
                        }
                    } 
                } 

                "-e" | "--editor" => {
                    let e = args.next().unwrap();
                    if !e.is_empty() {
                        config.editor = e.to_string();
                    }
                }

                "-h" | "--ignore-hidden" => {
                    let h = args.next().unwrap();
                    if !h.is_empty() {
                        config.ignore_hidden = false
                    }
                }

                _ => {
                    eprintln!("\"{}\" is not a valid argument", arg);
                    std::process::exit(1);
                }
            } 
        } else { 
            break;
        }
    }

    config
}

fn main() {
    let mut args: Vec<String> = args().skip(1).collect();
    let config = parse_args(args);
    println!("{:?}", config);

    // Command::new("fd").args(["-a", "-i", "--base-directory", home]).spawn().unwrap();
}
