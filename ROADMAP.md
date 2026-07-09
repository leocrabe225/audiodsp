# Roadmap

The goal is a small audio processor with a clean, tested core. It reads a WAV
file, applies one or more effects to the samples, and writes the result to a new
WAV file. Processing is offline, meaning the whole file is loaded and processed
at once rather than streamed in real time.

Items are listed roughly in build order. Checked items are done.

- [ ] **WAV read and write.** Load a WAV file into memory, write it back out
  unchanged, and confirm the output is sample accurate against the input.
  - [x] Convert audio to and from the internal format, verified lossless.
  - [ ] Read and write WAV files on disk.
- [ ] **Gain.** Scale the amplitude of every sample by a constant factor.
  Covered by tests for unity gain (output equals input), zero gain (silence),
  and a doubling factor, including the handling of values that would clip.
- [ ] **Effect model.** A single shared representation for effects so new ones
  can be added and composed without touching the rest of the pipeline.
- [ ] **Delay and echo.** Mix in a delayed and attenuated copy of the signal.
  Tested by feeding a single impulse and checking that a second, quieter impulse
  appears at the expected sample offset.
- [ ] **Low pass filter.** A basic filter across the sample stream, verified
  against known inputs.
- [ ] **Command line interface.** Select and chain multiple effects from the
  command line along with their parameters.
- [ ] **Block processing.** Process the audio in fixed size chunks instead of
  all at once. This is closer to how real time audio systems hand data to a
  processing callback, and is a step toward that model.
- [ ] **Stereo support.** Handle interleaved multi channel audio in addition to
  mono.
