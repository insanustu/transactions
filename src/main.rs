use state::State;
use std::{env, io::stdout, process::exit};

pub mod account;
pub mod errors;
pub mod fixed_precision;
pub mod record;
pub mod state;
pub mod transaction;

const fn usage() -> &'static str {
    "Usage:\n\ttransactions <CSV-file>"
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        std::eprintln!("Wrong amount of parameters {}", usage());
        exit(1)
    }
    let mut rdr = match csv::ReaderBuilder::new()
        .has_headers(true)
        .buffer_capacity(1024 * 1024)
        .trim(csv::Trim::All)
        .flexible(true)
        .from_path(args[1].clone())
    {
        Ok(reader) => reader,
        Err(err) => {
            std::eprintln!("Error while reading file {}: {err}", args[1]);
            exit(1);
        }
    };
    let mut state = State::default();
    state.process_data(&mut rdr);
    state.serialize_result(
        &mut csv::WriterBuilder::new()
            .has_headers(true)
            .buffer_capacity(1024 * 1024)
            .flexible(true)
            .from_writer(stdout()),
    );
}
