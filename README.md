# AudioDSP

A small command line tool for applying audio effects to WAV files, written in
Rust.

It loads a WAV file, runs the audio through one or more effects, and writes the
processed result to a new file. Processing is done offline over the whole file
rather than in real time.

## Status

Inception.
See ROADMAP.md for the full plan and the current state.

## Build

```
cargo build --release
```

## Usage
[NOT IMPLEMENTED]
```
cargo run -- input.wav output.wav --gain 2.0
```

This reads input.wav, applies a gain of 2.0, and writes output.wav. The
available effects and their flags are listed in ROADMAP.md as they land.

## Design

The code is split into three parts with clear boundaries.

- **dsp** holds the effects. Each effect is a pure transformation over a buffer
  of samples with no knowledge of files or the command line. This is the part
  that carries the tests.
- **io** handles reading and writing WAV files and converting between the file
  sample format and the internal format used for processing.
- **cli** parses arguments and connects the chosen effects to the input and
  output files. It stays thin.

Keeping the effects pure and free of input and output makes them easy to test.
Each effect has tests that pin its numerical behavior, so a known input is
expected to produce an exact known output.

## Testing
[NOT IMPLEMENTED]
```
cargo test
```

Tests focus on the dsp core. Where floating point output is compared, the
comparison uses a small tolerance rather than exact equality.

## A note on audio

For anyone new to audio code, the short version is that a mono signal is just a
list of numbers, one per sample, each holding the amplitude at that moment. The
sample rate is how many of those numbers make up one second. Most effects here
are plain arithmetic over that list. Gain multiplies each number by a factor.
Delay adds a shifted copy of the list back onto itself.

## License

MIT. See [LICENSE](LICENSE.txt).
