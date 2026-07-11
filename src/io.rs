use std::path::Path;

fn i16_to_f32(sample: i16) -> f32 {
    let f = sample as f32;

    f / 32768.0
}

fn f32_to_i16(sample: f32) -> i16 {
    (sample * 32768.0) as i16
}

fn i16_array_to_f32_vector(sample: &[i16]) -> Vec<f32> {
    sample.iter().map(|s| i16_to_f32(*s)).collect()
}

fn write_wav(path: &Path, samples: &[f32], sample_rate: u32) -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(path, spec)?;
    for s in samples {
        writer.write_sample(f32_to_i16(*s))?;
    }
    writer.finalize()?;
    Ok(())
}

fn read_wav(path: &Path) -> Result<(Vec<f32>, u32), hound::Error> {
    let reader = hound::WavReader::open(path)?;
    let sample_rate = reader.spec().sample_rate;
    let samples = reader
        .into_samples::<i16>()
        .collect::<Result<Vec<_>, _>>()?;
    Ok((i16_array_to_f32_vector(&samples), sample_rate))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn half_scale_i16_converts_to_half_f32() {
        let converted = i16_to_f32(16384);
        assert!((converted - 0.5).abs() < 1e-6);
    }

    #[test]
    fn zero_i16_converts_to_zero_f32() {
        let converted = i16_to_f32(0);
        assert_eq!(converted, 0.0);
    }

    #[test]
    fn min_i16_converts_to_minus_one_exactly_f32() {
        let converted = i16_to_f32(i16::MIN);
        assert_eq!(converted, -1.0);
    }

    #[test]
    fn max_i16_converts_to_almost_one_f32() {
        let converted = i16_to_f32(i16::MAX);
        assert!(converted < 1.0);
    }

    #[test]
    fn half_scale_f32_converts_to_half_i16() {
        let converted = f32_to_i16(0.5);
        assert_eq!(converted, 16384);
    }

    #[test]
    fn zero_f32_converts_to_zero_i16() {
        let converted = f32_to_i16(0.0);
        assert_eq!(converted, 0);
    }

    #[test]
    fn roadtrip_keeps_initial_value() {
        for x in i16::MIN..=i16::MAX {
            let converted = i16_to_f32(x);
            let converted_back = f32_to_i16(converted);
            assert_eq!(converted_back, x, "round-trip broke for input {x}");
        }
    }

    #[test]
    fn one_f32_value_clamped_to_max_i16() {
        let converted = f32_to_i16(1.0);
        assert_eq!(converted, 32767);
    }

    #[test]
    fn high_f32_value_clamped_to_max_i16() {
        let converted = f32_to_i16(1.5);
        assert_eq!(converted, 32767);
    }

    #[test]
    fn low_f32_value_clamped_to_min_i16() {
        let converted = f32_to_i16(-1.5);
        assert_eq!(converted, -32768);
    }

    #[test]
    fn nan_f32_converts_to_zero_i16() {
        let converted = f32_to_i16(f32::NAN);
        assert_eq!(converted, 0);
    }

    #[test]
    fn inf_f32_converts_to_max_i16() {
        let converted = f32_to_i16(f32::INFINITY);
        assert_eq!(converted, 32767);
    }

    #[test]
    fn neg_inf_f32_converts_to_min_i16() {
        let converted = f32_to_i16(f32::NEG_INFINITY);
        assert_eq!(converted, -32768);
    }

    #[test]
    fn pos_f32_that_should_round_up_actually_truncates_i16() {
        let converted = f32_to_i16(1000.9375 / 32768.0); // 1000.9375 (1000 + 15 / 16), rounding gives 1001, truncation gives 1000
        assert_eq!(converted, 1000);
    }

    #[test]
    fn neg_f32_that_should_round_up_actually_truncates_i16() {
        let converted = f32_to_i16(-1000.9375 / 32768.0); // -1000.9375 (1000 + 15 / 16), rounding gives 1001, truncation gives -1000
        assert_eq!(converted, -1000);
    }

    #[test]
    fn i16_array_returns_matching_f32_vector() {
        let converted = i16_array_to_f32_vector(&[0, 16384, i16::MIN]);
        assert_eq!(converted, vec![0.0, 0.5, -1.0]);
    }

    #[test]
    fn wav_round_trip_preserves_samples_and_rate() {
        let samples: Vec<f32> = vec![0.0, 0.5, -0.5, -1.0]; //exact powers of two for equality check.
        let sample_rate = 44100;

        let dir = tempfile::tempdir().expect("should create a temp dir");
        let path = dir.path().join("round_trip.wav");

        write_wav(&path, &samples, sample_rate).expect("write should succeed");
        let (read_samples, read_rate) = read_wav(&path).expect("read should succeed");

        assert_samples_and_rate(&read_samples, &samples, read_rate, sample_rate);
    }

    #[test]
    fn wav_round_trip_preserves_rate_even_when_empty() {
        let samples: Vec<f32> = vec![];
        let sample_rate = 44100;

        let dir = tempfile::tempdir().expect("should create a temp dir");
        let path = dir.path().join("round_trip.wav");

        write_wav(&path, &samples, sample_rate).expect("write should succeed");
        let (read_samples, read_rate) = read_wav(&path).expect("read should succeed");

        assert_samples_and_rate(&read_samples, &samples, read_rate, sample_rate);
    }

    #[test]
    fn wav_round_trip_clamps_to_i16_extremes() {
        let samples: Vec<f32> = vec![-1.0, 1.0];
        let expected_samples: Vec<f32> = vec![-1.0, 32767.0 / 32768.0];
        let sample_rate = 44100;

        let dir = tempfile::tempdir().expect("should create a temp dir");
        let path = dir.path().join("round_trip.wav");

        write_wav(&path, &samples, sample_rate).expect("write should succeed");
        let (read_samples, read_rate) = read_wav(&path).expect("read should succeed");

        assert_samples_and_rate(&read_samples, &expected_samples, read_rate, sample_rate);
    }

    #[test]
    fn wav_read_from_fixture_gets_samples_and_rate() {
        let expected_samples: Vec<f32> = vec![0.0, 0.5, -0.5, -1.0];
        let expected_sample_rate = 8000;

        let path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/reference.wav"
        ));

        let (read_samples, read_rate) = read_wav(path).expect("read should succeed");

        assert_samples_and_rate(
            &read_samples,
            &expected_samples,
            read_rate,
            expected_sample_rate,
        );
    }

    #[test]
    fn wav_read_from_non_existing_path_return_io_error() {
        let path = Path::new("This path so does not exist.");

        match read_wav(path).unwrap_err() {
            hound::Error::IoError(_) => {}
            other => panic!("expected a IoError, got {other:?}"),
        };
    }

    #[test]
    fn wav_read_from_malformed_file_return_format_error() {
        let path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/malformed.wav"
        ));

        match read_wav(path).unwrap_err() {
            hound::Error::FormatError(_) => {}
            other => panic!("expected a FormatError, got {other:?}"),
        };
    }

    #[track_caller]
    fn assert_samples_and_rate(got: &[f32], want: &[f32], read_rate: u32, wanted_rate: u32) {
        assert_eq!(read_rate, wanted_rate, "sample rate must be read properly");

        assert_eq!(got.len(), want.len(), "must preserve the sample count");

        for (&got, &want) in got.iter().zip(want) {
            assert!(got == want, "samples differs: wanted {want}, got {got}",);
        }
    }
}
