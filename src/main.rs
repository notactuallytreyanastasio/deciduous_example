use std::env;
use std::fs;
use std::process;

use rwc::{count_lines, count_words};

fn print_usage(program: &str) {
    eprintln!("Usage: {} [-l] <file>", program);
    eprintln!("  -l    Count lines only");
    eprintln!("  Without flags: count lines, words, and bytes");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    if args.len() < 2 {
        print_usage(program);
        process::exit(1);
    }

    let (lines_only, filename) = if args[1] == "-l" {
        if args.len() < 3 {
            print_usage(program);
            process::exit(1);
        }
        (true, &args[2])
    } else {
        (false, &args[1])
    };

    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}: {}: {}", program, filename, e);
            process::exit(1);
        }
    };

    if lines_only {
        println!("{} {}", count_lines(&contents), filename);
    } else {
        let lines = count_lines(&contents);
        let words = count_words(&contents);
        let bytes = contents.len();
        println!("{} {} {} {}", lines, words, bytes, filename);
    }
}
