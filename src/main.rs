use audiodsp::cli;
use audiodsp::dsp;
use audiodsp::io;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let config = cli::parse_args(&args).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    let (samples, sample_rate) = io::read_wav(Path::new(&config.input)).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    let result = dsp::apply_chain(&config.effects, &samples);

    io::write_wav(Path::new(&config.output), &result, sample_rate).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    eprintln!("File processed successfully!");
}
