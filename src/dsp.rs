mod echo;
mod gain;
mod low_pass;
#[cfg(test)]
mod test_support;

#[derive(Debug, PartialEq)]
pub enum Effect {
    Gain { factor: f32 },
    Echo { delay: usize, factor: f32 },
    LowPass { alpha: f32 },
}

impl Effect {
    fn apply(&self, samples: &[f32]) -> Vec<f32> {
        match self {
            Effect::Gain { factor } => gain::gain(samples, *factor),
            Effect::Echo { delay, factor } => echo::echo(samples, *delay, *factor),
            Effect::LowPass { alpha } => low_pass::low_pass(samples, *alpha),
        }
    }
}

pub fn apply_chain(effects: &[Effect], samples: &[f32]) -> Vec<f32> {
    effects
        .iter()
        .fold(samples.to_vec(), |acc, effect| effect.apply(&acc))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::assert_close;

    #[test]
    fn a_chain_applies_each_effect_in_sequence() {
        let input: Vec<f32> = vec![1.0, 0.0, 0.0, 0.0];
        let expected: Vec<f32> = vec![2.0, 0.0, 1.0, 0.0];

        let effects = vec![
            Effect::Gain { factor: 2.0 },
            Effect::Echo {
                delay: 2,
                factor: 0.5,
            },
        ];

        let output = apply_chain(&effects, &input);

        assert_close(&output, &expected);
    }

    #[test]
    fn an_empty_chain_returns_unchanged_and_does_not_panic() {
        let input: Vec<f32> = vec![1.0, 0.0, 0.0, 0.0];
        let expected: Vec<f32> = vec![1.0, 0.0, 0.0, 0.0];

        let effects = vec![];

        let output = apply_chain(&effects, &input);

        assert_close(&output, &expected);
    }

    #[test]
    fn an_empty_buffer_returns_empty_and_does_not_panic() {
        let input: Vec<f32> = vec![];
        let expected: Vec<f32> = vec![];

        let effects = vec![
            Effect::Gain { factor: 2.0 },
            Effect::Echo {
                delay: 2,
                factor: 0.5,
            },
        ];

        let output = apply_chain(&effects, &input);

        assert_close(&output, &expected);
    }

    #[test]
    fn a_single_effect_chain_behaves_as_the_underlying_effect() {
        let input: Vec<f32> = vec![3.0, 0.5, 1.0, 0.0];

        let output1 = gain::gain(&input, 2.0);

        let effects = vec![Effect::Gain { factor: 2.0 }];

        let output2 = apply_chain(&effects, &input);

        assert_close(&output1, &output2);
    }

    #[test]
    fn a_single_low_pass_effect_chain_behaves_as_the_function() {
        let input: Vec<f32> = vec![3.0, 0.5, 1.0, 0.0];

        let output1 = low_pass::low_pass(&input, 0.5);

        let effects = vec![Effect::LowPass { alpha: 0.5 }];

        let output2 = apply_chain(&effects, &input);

        assert_close(&output1, &output2);
    }
}
