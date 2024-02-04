use std::{error::Error, fmt::Display, path::Path};

pub struct ParsedArgs {
    pub input_paths: Vec<String>,
    pub include_dirs: Vec<String>,
    pub output_dir: String,
}

#[derive(Debug)]
pub enum ParseArgsError {
    MissingArgument(String),
    FileNotFound(String),
}

impl Display for ParseArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingArgument(s) => f.write_str(s),
            Self::FileNotFound(s) => f.write_str(s),
        }
    }
}

impl Error for ParseArgsError {}

pub fn parse_command_line_args() -> Result<ParsedArgs, ParseArgsError> {
    let mut parsed_args = ParsedArgs {
        input_paths: vec![],
        include_dirs: vec!["".into()],
        output_dir: String::new(),
    };
    let mut args = std::env::args();
    if args.len() == 0 {
        panic!("No source files provided");
    }
    args.next();
    loop {
        match args.next() {
            None => break,
            Some(arg) => {
                match arg.to_lowercase().as_str() {
                    "-i" => {
                        let Some(include_dir) = args.next() else {
                            return Err(ParseArgsError::MissingArgument(format!("Directory required after option `-i`")));
                        };
                        parsed_args.include_dirs.push(include_dir);
                    },
                    "-o" => {
                        let Some(output_dir) = args.next() else {
                            return Err(ParseArgsError::MissingArgument(format!("Directory required after option `-i`")));
                        };
                        parsed_args.output_dir = output_dir;
                    },
                    _ => {
                        if Path::new(&arg).is_file() {
                            parsed_args.input_paths.push(arg);
                        } else {
                            return Err(ParseArgsError::FileNotFound(format!("File `{arg}` cannot be located")));
                        }
                    },
                }
            }
        }
    }
    Ok(parsed_args)
}
