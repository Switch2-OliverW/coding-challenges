use clap::Parser;
use std::{
    fs,
    io::{self, BufRead, IsTerminal},
    process::exit,
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'c')]
    bytes: bool,
    #[arg(short = 'm')]
    chars: bool,
    #[arg(short = 'l')]
    lines: bool,
    #[arg(short = 'w')]
    words: bool,
    file_path: Option<String>,
}

#[derive(Debug)]
struct Data {
    path: String,
    contents: String,
}
impl Data {
    fn new_from_file(path: String) -> Self {
        Data {
            path: path.clone(),
            contents: fs::read_to_string(path).unwrap(),
        }
    }

    fn new_from_data(data: String) -> Self {
        Data {
            path: String::new(),
            contents: data,
        }
    }

    fn get_bytes(&self) -> usize {
        self.contents.len()
    }

    fn get_lines(&self) -> usize {
        self.contents.lines().collect::<Vec<&str>>().len()
    }

    fn get_chars(&self) -> usize {
        self.contents.chars().collect::<Vec<char>>().len()
    }

    fn get_words(&self) -> usize {
        self.contents
            .split_whitespace()
            .collect::<Vec<&str>>()
            .len()
    }
}

fn main() {
    let args = Args::parse();

    let input = io::stdin();

    let data = match args.file_path {
        Some(path) => Data::new_from_file(path),
        None => match input.is_terminal() {
            true => {
                println!("No target provided");
                exit(0);
            }
            false => Data::new_from_data(
                input
                    .lock()
                    .lines()
                    .map_while(Result::ok)
                    .collect::<Vec<String>>()
                    .join("\n"),
            ),
        },
    };

    let mut output: Vec<String> = vec![];

    if args.bytes {
        output.push(data.get_bytes().to_string());
    }

    if args.chars {
        output.push(data.get_chars().to_string());
    }

    if args.lines {
        output.push(data.get_lines().to_string());
    }

    if args.words {
        output.push(data.get_words().to_string());
    }

    if output.is_empty() {
        output.push(data.get_lines().to_string());
        output.push(data.get_words().to_string());
        output.push(data.get_bytes().to_string());
    }

    println!("{} {}", output.join(" "), data.path);
}
