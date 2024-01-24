use std::{ fmt, env, io::{self, Write}, process };
mod query_resolver;
mod test;
use csvr::CSVFile;
use query_resolver::query_resolver;

#[derive(Debug)]
enum FileLoadError {
    FileNameMissing,
    ExtraArguments,
}

impl fmt::Display for FileLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileLoadError::FileNameMissing => write!(f, "csvr: missing argument: filename"),
            FileLoadError::ExtraArguments => write!(f, "csvr: more arguments than required"),
        }
    }
}

fn load_file(load_file_query: Vec<String>) -> Result<CSVFile, FileLoadError> {
    if load_file_query.len() < 2 { return Err(FileLoadError::FileNameMissing) }
    if load_file_query.len() > 2 { return Err(FileLoadError::ExtraArguments) }

    let file_name = &load_file_query[1];
    let file = CSVFile::new(file_name);
    println!("\x1b[32mSuccessfully Loaded File: \x1b[33m{}\x1b[0m", file_name);
    Ok(file)
}

fn main() {
    let load_file_query: Vec<String> = env::args().collect();
    let load_result = load_file(load_file_query);
    if let Err(load_file_error) = &load_result {
        eprintln!("\x1b[31m{}\x1b[0m", load_file_error);
        process::exit(1);
    }

    let mut file = load_result.unwrap();

    loop {
        print!(">>>  ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);

        if input == "exit\n".to_owned() { break; }
        else { query_resolver(input.clone(), &mut file); }
    }
}
