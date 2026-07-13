pub(super) fn gain(samples: &[f32], factor: f32) -> Vec<f32> {
    samples.iter().map(|&s| s * factor).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsp::test_support::assert_close;

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
}
