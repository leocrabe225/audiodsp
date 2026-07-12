use std::path::Path;

pub mod cli;
pub mod dsp;
pub mod io;

pub fn run(config: &cli::Config) -> Result<(), hound::Error> {
    let (samples, sample_rate) = io::read_wav(Path::new(&config.input))?;

    let result = dsp::apply_chain(&config.effects, &samples);

    io::write_wav(Path::new(&config.output), &result, sample_rate)?;
    Ok(())
}
