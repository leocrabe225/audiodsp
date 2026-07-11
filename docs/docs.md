## Sample format

- Dithering is out of scope for this project.
- Effects never clamp internally. A sample may exceed ±1.0 mid-chain;
  saturation to the i16 range happens exactly once, at the f32->i16 write
  boundary (the `as i16` cast, saturating since Rust 1.45). Clamping mid-chain
  would discard signal a later effect could pull back into range.

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