# Test fixtures

Known-good audio files generated **independently of this crate's own WAV code**,
so tests can check the reader against an external oracle rather than against
our own writer (which a round-trip test can't do).

## reference.wav

- Format: mono, signed 16-bit PCM, **8000 Hz** (deliberately not 44100, so a
  reader that ignores the rate gets caught).
- Samples (i16): `[0, 16384, -16384, -32768]`
- As internal f32: `[0.0, 0.5, -0.5, -1.0]`

Generated with Python's stdlib `wave` module:

```python
import wave, struct
samples = [0, 16384, -16384, -32768]
with wave.open('tests/fixtures/reference.wav', 'wb') as w:
    w.setnchannels(1)
    w.setsampwidth(2)
    w.setframerate(8000)
    w.writeframes(b''.join(struct.pack('<h', s) for s in samples))
```

## malformed.wav

The same bytes as `reference.wav`, but with the RIFF form-type tag at bytes
8..12 clobbered from `WAVE` to `WAVX`. That single change makes it no longer a
valid WAVE container, so a reader must reject it with a *format* error (not an
I/O error), which is what the malformed-file test pins.

Regenerate deterministically:

```python
import wave, struct, io

# Rebuild the good bytes in memory (same as reference.wav).
buf = io.BytesIO()
samples = [0, 16384, -16384, -32768]
with wave.open(buf, 'wb') as w:
    w.setnchannels(1)
    w.setsampwidth(2)
    w.setframerate(8000)
    w.writeframes(b''.join(struct.pack('<h', s) for s in samples))

data = bytearray(buf.getvalue())
assert data[8:12] == b'WAVE'   # the form-type tag
data[8:12] = b'WAVX'           # clobber it

with open('tests/fixtures/malformed.wav', 'wb') as f:
    f.write(data)
```
