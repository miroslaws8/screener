use std::env;

pub struct Args {
    pub url: String,
    pub wait_for: String,
}

pub fn parse() -> Args {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <URL> <WAIT_FOR>", args[0]);
        std::process::exit(1);
    }

    Args {
        url: args[1].clone(),
        wait_for: args[2].clone(),
    }
}