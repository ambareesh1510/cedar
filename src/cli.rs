use std::path::Path;

pub struct ParsedArgs {
    pub source_path: Vec<String>,
}
pub fn parse_command_line_args() -> ParsedArgs {
    let mut parsed_args = ParsedArgs {
        source_path: vec![],
    };
    let args = std::env::args();
    if args.len() == 0 {
        panic!("No source files provided");
    }
    for arg in args.skip(1) {
        if Path::new(&arg).is_file() {
            parsed_args.source_path.push(arg);
        } else {
            panic!("Invalid file path: `{arg}`");
        }
    }
    parsed_args
}
