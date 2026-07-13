pub(super) fn low_pass(samples: &[f32], alpha: f32) -> Vec<f32> {
    samples
        .iter()
        .scan(0.0, |state, &sample| {
            *state = *state * (1.0 - alpha) + sample * alpha;
            Some(*state)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsp::test_support::assert_close;

    #[test]
    fn low_pass_decays_geometrically() {
        let input: Vec<f32> = vec![1.0, 0.0, 0.0, 0.0];
        let expected: Vec<f32> = vec![0.5, 0.25, 0.125, 0.0625];

        let output = low_pass(&input, 0.5);

        assert_close(&output, &expected);
    }

    #[test]
    fn low_pass_alpha_one_passes_through_unchanged() {
        let input: Vec<f32> = vec![1.0, 0.3, -0.2, 0.5];
        let expected: Vec<f32> = vec![1.0, 0.3, -0.2, 0.5];

        let output = low_pass(&input, 1.0);

        assert_close(&output, &expected);
    }

    #[test]
    fn low_pass_alpha_zero_outputs_all_zero() {
        let input: Vec<f32> = vec![1.0, 0.0, 0.5, 0.1];
        let expected: Vec<f32> = vec![0.0, 0.0, 0.0, 0.0];

        let output = low_pass(&input, 0.0);

        assert_close(&output, &expected);
    }

    #[test]
    fn low_pass_asymmetric_alpha_weights_properly() {
        let input: Vec<f32> = vec![1.0, 0.0];
        let expected: Vec<f32> = vec![0.25, 0.1875];

        let output = low_pass(&input, 0.25);

        assert_close(&output, &expected);
    }

    #[test]
    fn low_pass_constant_input_settles_toward_that_constant() {
        let input: Vec<f32> = vec![0.5, 0.5, 0.5, 0.5];
        let expected: Vec<f32> = vec![0.25, 0.375, 0.4375, 0.46875];

        let output = low_pass(&input, 0.5);

        assert_close(&output, &expected);
    }

    #[test]
    fn low_pass_empty_buffer_returns_empty_and_does_not_panic() {
        let input: Vec<f32> = vec![];
        let expected: Vec<f32> = vec![];

        let output = low_pass(&input, 0.5);

        assert_close(&output, &expected);
    }
}
