use audiodsp::cli;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let config = cli::parse_args(&args).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    audiodsp::run(&config).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    eprintln!("File processed successfully!");
}
