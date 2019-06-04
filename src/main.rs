use std::{
    fs,
    io::{self, Write},
};

// (Buf) Uncomment these lines to have the output buffered, this can provide
// better performance but is not always intuitive behaviour.
// use std::io::BufWriter;

use structopt::StructOpt;

// Our CLI arguments. (help and version are automatically generated)
// Documentation on how to use:
// https://docs.rs/structopt/0.2.10/structopt/index.html#how-to-derivestructopt
#[derive(StructOpt, Debug)]
struct Cli {
    // The pattern we want to look for.
    pattern: String,
    // The path of the file we want to look at.
    path: String,
}

fn main() {
    let args = Cli::from_args();
    let contents = fs::read_to_string(&args.path)
        .expect("Could not read file.");
    let mut stdout = io::stdout();
    // (Buf) Wraps stdout in a buffer.
    // let mut stdout = BufWriter::new(stdout);

    for (line_no, line) in contents.lines().enumerate() {
        if line.contains(&args.pattern) {
            writeln!(stdout, "{}: {}", line_no + 1, line);
        }
    }
}

