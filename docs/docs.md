## Sample format

- Dithering is out of scope for this project.
- Effects never clamp internally. A sample may exceed ±1.0 mid-chain;
  saturation to the i16 range happens exactly once, at the f32->i16 write
  boundary (the `as i16` cast, saturating since Rust 1.45). Clamping mid-chain
  would discard signal a later effect could pull back into range.

## Effects

- Effects are a closed `enum Effect`, not a `trait` + dynamic dispatch. The
  effect set is small and wholly owned by this crate, so compiler-enforced
  exhaustive `match` (add a variant -> every match must handle it or it won't
  compile) beats open extensibility. Revisit only if effects must be loaded
  open / at runtime (plugins).
- An effect whose output at sample `n` reads an *earlier* sample (echo reads
  `n - delay`) must stay out-of-place:
  read the input slice, write a fresh `Vec`. An in-place `&mut [f32]` pass going
  forward overwrites `samples[n - delay]` before index `n` reads it, silently
  turning one echo into a decaying feedback loop — wrong numbers, no panic. The
  per-call allocation is correctness, not overhead. (`gain` is in-place-safe:
  output `n` depends only on input `n`.)
- `Effect::LowPass { alpha }` is one-pole IIR (`y[n] = α·x[n] + (1-α)·y[n-1]`),
  carrying its previous *output* in the iterator's `scan` state — it never indexes
  the input buffer, unlike echo. `alpha` is deliberately not validated (scoped
  week). Note the trap for later: unlike gain/echo (feedforward — a bad factor
  only scales output, bounded), this is feedback with a pole at `1-α`. It's stable
  only for `0 < α < 2`, an actual low-pass only for `α ∈ (0, 1]`; outside that the
  output diverges (grows unbounded each sample), it does not clip. If hardened,
  prefer a validating `Alpha` newtype (illegal states unrepresentable) over a
  runtime clamp/assert.

## CLI

- The argument parser is hand-rolled (`cli::parse_args`), not `clap`.
  Grammar is positional: `<in> <out> [<effect> <params>...]`, effects applied 
  in argument order (pinned by `cli` tests).
  Reach for `clap` only if the surface grows flags/subcommands.
- Errors are handled manually in `main`: `eprintln!("{e}")` (via `Display for
  ParseError`) then `std::process::exit(1)`. Deliberately *not*
  `main() -> Result<(), Box<dyn Error>>` — that path renders the error with `Debug`,
  not `Display`, discarding the crafted message. No unified app-error enum
  (`Parse | Io`) either: two small handling sites in `main` beat the machinery for a
  week-scoped tool.
- `lib::run(&Config) -> Result<(), hound::Error>` is the read->apply->write seam,
  called by both `main` and the integration test so the wiring is tested rather than
  duplicated. Return type is the concrete `hound::Error`, not `Box<dyn Error>`: every
  failure in `run` is a `hound::Error`, so erasing the type would trade real precision
  for flexibility that isn't needed. Widen only when `run` can fail more than one way.
  
## Tests

- Never `==` on computed floats. Use `assert!((a - b).abs() < 1e-6)` for
  tolerance, and `assert!(a < b)` for inequality-shaped truths.
- `assert_eq!` is correct for exact, integer-valued results (integers, and
  exactly-representable/exactly-computed floats like `0.0`/`-1.0`) — it also
  prints both sides on failure, and clippy's `float_cmp` permits these.
- Write an exact non-trivial expected value as the arithmetic that shows why
  it's exact (`32767.0 / 32768.0`), never a hand-rounded decimal (`0.9999695`)
  that only happens to round to the same bits.
- Small finite input domains: prefer an exhaustive loop with a diagnostic
  message (`assert_eq!(got, want, "... {x}")`) over sampling.
- `proptest` is reserved for large/continuous domains (e.g. the low-pass
  filter); it is not yet a dependency.