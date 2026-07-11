fn gain(samples: &[f32], factor: f32) -> Vec<f32> {
    samples.iter().map(|&s| s * factor).collect()
}

fn echo(samples: &[f32], delay: usize, factor: f32) -> Vec<f32> {
    samples
        .iter()
        .enumerate()
        .map(|(n, &x)| {
            if let Some(src) = n.checked_sub(delay) {
                x + samples[src] * factor
            } else {
                x
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gain_doubles_every_sample() {
        let input: Vec<f32> = vec![0.1, -0.2, 0.3];
        let expected: Vec<f32> = vec![0.2, -0.4, 0.6];

        let output = gain(&input, 2.0);

        assert_close(&output, &expected);
    }

    #[test]
    fn unity_gain_does_not_change_samples() {
        let input: Vec<f32> = vec![0.1, -0.2, 0.3];
        let expected: Vec<f32> = vec![0.1, -0.2, 0.3];

        let output = gain(&input, 1.0);

        assert_close(&output, &expected);
    }

    #[test]
    fn zero_gain_produces_silence() {
        let input: Vec<f32> = vec![0.1, -0.2, 0.3];
        let expected: Vec<f32> = vec![0.0, 0.0, 0.0];

        let output = gain(&input, 0.0);

        assert_close(&output, &expected);
    }

    #[test]
    fn fractional_gain_attenuates() {
        let input: Vec<f32> = vec![0.1, -0.2, 0.3];
        let expected: Vec<f32> = vec![0.05, -0.1, 0.15];

        let output = gain(&input, 0.5);

        assert_close(&output, &expected);
    }

    #[test]
    fn negative_gain_inverts_polarity() {
        let input: Vec<f32> = vec![0.1, -0.2, 0.3];
        let expected: Vec<f32> = vec![-0.1, 0.2, -0.3];

        let output = gain(&input, -1.0);

        assert_close(&output, &expected);
    }

    #[test]
    fn gain_with_empty_buffers_outputs_empty_and_does_not_panic() {
        let input: Vec<f32> = vec![];
        let expected: Vec<f32> = vec![];

        let output = gain(&input, 2.0);

        assert_close(&output, &expected);
    }

    #[test]
    fn gain_does_not_clamp() {
        let input: Vec<f32> = vec![0.1, 0.2, 0.7];
        let expected: Vec<f32> = vec![0.2, 0.4, 1.4];

        let output = gain(&input, 2.0);

        assert_close(&output, &expected);
    }

    #[test]
    fn impulse_produces_a_delayed_attenuated_echo() {
        let input: Vec<f32> = vec![1.0, 1.0, 0.0, 0.0, 0.0];
        let expected: Vec<f32> = vec![1.0, 1.0, 0.5, 0.5, 0.0];

        let output = echo(&input, 2, 0.5);

        assert_close(&output, &expected);
    }

    #[test]
    fn zero_echo_leaves_the_signal_unchanged() {
        let input: Vec<f32> = vec![1.0, 0.0, 1.0, 0.0, 0.0];
        let expected: Vec<f32> = vec![1.0, 0.0, 1.0, 0.0, 0.0];

        let output = echo(&input, 2, 0.0);

        assert_close(&output, &expected);
    }

    #[test]
    fn echo_with_zero_delay_adds_scaled_signal_onto_itself() {
        let input: Vec<f32> = vec![1.0, 0.5, 2.0];
        let expected: Vec<f32> = vec![1.5, 0.75, 3.0];

        let output = echo(&input, 0, 0.5);

        assert_close(&output, &expected);
    }

    #[test]
    fn echo_empty_buffer_returns_empty_and_does_not_panic() {
        let input: Vec<f32> = vec![];
        let expected: Vec<f32> = vec![];

        let output = echo(&input, 2, 0.5);

        assert_close(&output, &expected);
    }

    #[test]
    fn echo_does_not_affect_samples_before_delay_and_does_not_extend_length() {
        let input: Vec<f32> = vec![1.0, 0.5];
        let expected: Vec<f32> = vec![1.0, 0.5];

        let output = echo(&input, 2, 0.5);

        assert_close(&output, &expected);
    }

    #[test]
    fn echo_landing_on_non_zero_adds_does_not_replace() {
        let input: Vec<f32> = vec![1.0, 0.0, 0.2];
        let expected: Vec<f32> = vec![1.0, 0.0, 0.7];

        let output = echo(&input, 2, 0.5);

        assert_close(&output, &expected);
    }

    #[test]
    fn echo_does_not_clamp() {
        let input: Vec<f32> = vec![1.0, 0.0, 1.0];
        let expected: Vec<f32> = vec![1.0, 0.0, 1.5];

        let output = echo(&input, 2, 0.5);

        assert_close(&output, &expected);
    }

    #[track_caller]
    fn assert_close(got: &[f32], want: &[f32]) {
        assert_eq!(got.len(), want.len(), "must preserve the sample count");

        for (&got, &want) in got.iter().zip(want) {
            assert!(
                (got - want).abs() < 1e-6,
                "sample differs: wanted {want}, got {got}",
            );
        }
    }
}
