## Sample format

- Dithering is out of scope for this project.

## Tests

- Never `==` on computed floats. Use `assert!((a - b).abs() < 1e-6)` for
  tolerance, and `assert!(a < b)` for inequality-shaped truths.
- `assert_eq!` is correct for exact, integer-valued results (integers, and
  exactly-representable/exactly-computed floats like `0.0`/`-1.0`) — it also
  prints both sides on failure, and clippy's `float_cmp` permits these.
- Small finite input domains: prefer an exhaustive loop with a diagnostic
  message (`assert_eq!(got, want, "... {x}")`) over sampling.
- `proptest` is reserved for large/continuous domains (e.g. the low-pass
  filter); it is not yet a dependency.