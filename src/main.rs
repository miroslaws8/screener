use std::error::Error;

mod modules {
    pub mod args_parser;
    pub mod screener;
}

use modules::args_parser::parse;
use modules::screener::capture;

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse();
    capture(args)?;

    Ok(())
}