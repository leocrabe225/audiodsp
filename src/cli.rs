use crate::dsp::Effect;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    MissingInput,
    MissingOutput,
    UnknownEffect(String),
    MissingParam(String),
    InvalidNumber(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::MissingInput => write!(f, "the input is missing"),
            ParseError::MissingOutput => write!(f, "the output is missing"),
            ParseError::UnknownEffect(effect) => {
                write!(f, "unknown effect: '{effect}'")
            }
            ParseError::MissingParam(param) => {
                write!(f, "missing param for '{param}'")
            }
            ParseError::InvalidNumber(number) => {
                write!(f, "invalid number: '{number}'")
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub input: String,
    pub output: String,
    pub effects: Vec<Effect>,
}

fn parse_num<T: FromStr>(token: &str) -> Result<T, ParseError> {
    token
        .parse()
        .map_err(|_| ParseError::InvalidNumber(token.into()))
}

pub fn parse_args(args: &[String]) -> Result<Config, ParseError> {
    let mut it = args.iter();
    let input = it.next().ok_or(ParseError::MissingInput)?;
    let output = it.next().ok_or(ParseError::MissingOutput)?;
    let mut effects: Vec<Effect> = vec![];

    while let Some(command) = it.next() {
        let mut next_token = || {
            it.next()
                .ok_or_else(|| ParseError::MissingParam(command.clone()))
        };
        match command.as_str() {
            "gain" => effects.push(Effect::Gain {
                factor: parse_num(next_token()?)?,
            }),
            "echo" => effects.push(Effect::Echo {
                delay: parse_num(next_token()?)?,
                factor: parse_num(next_token()?)?,
            }),
            "lowpass" => effects.push(Effect::LowPass {
                alpha: parse_num(next_token()?)?,
            }),
            other => return Err(ParseError::UnknownEffect(other.into())),
        }
    }

    Ok(Config {
        input: input.clone(),
        output: output.clone(),
        effects,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_two_paths_with_no_effects() {
        let args: Vec<String> = vec!["in.wav".into(), "out.wav".into()];

        let config = parse_args(&args).expect("well-formed args should parse");

        assert_eq!(
            config,
            Config {
                input: "in.wav".into(),
                output: "out.wav".into(),
                effects: vec![],
            }
        );
    }

    #[test]
    fn parses_gain_effect() {
        let args: Vec<String> = vec![
            "in.wav".into(),
            "out.wav".into(),
            "gain".into(),
            "2.0".into(),
        ];

        let config = parse_args(&args).expect("well-formed args should parse");

        assert_eq!(
            config,
            Config {
                input: "in.wav".into(),
                output: "out.wav".into(),
                effects: vec![Effect::Gain { factor: 2.0 }],
            }
        );
    }

    #[test]
    fn parses_echo_effect() {
        let args: Vec<String> = vec![
            "in.wav".into(),
            "out.wav".into(),
            "echo".into(),
            "2".into(),
            "0.4".into(),
        ];

        let config = parse_args(&args).expect("well-formed args should parse");

        assert_eq!(
            config,
            Config {
                input: "in.wav".into(),
                output: "out.wav".into(),
                effects: vec![Effect::Echo {
                    delay: 2,
                    factor: 0.4
                }],
            }
        );
    }

    #[test]
    fn parses_lowpass_effect() {
        let args: Vec<String> = vec![
            "in.wav".into(),
            "out.wav".into(),
            "lowpass".into(),
            "0.4".into(),
        ];

        let config = parse_args(&args).expect("well-formed args should parse");

        assert_eq!(
            config,
            Config {
                input: "in.wav".into(),
                output: "out.wav".into(),
                effects: vec![Effect::LowPass { alpha: 0.4 }],
            }
        );
    }

    #[test]
    fn parses_several_effects_in_order() {
        let args: Vec<String> = vec![
            "in.wav".into(),
            "out.wav".into(),
            "lowpass".into(),
            "0.4".into(),
            "echo".into(),
            "2".into(),
            "0.4".into(),
            "gain".into(),
            "2.0".into(),
        ];

        let config = parse_args(&args).expect("well-formed args should parse");

        assert_eq!(
            config,
            Config {
                input: "in.wav".into(),
                output: "out.wav".into(),
                effects: vec![
                    Effect::LowPass { alpha: 0.4 },
                    Effect::Echo {
                        delay: 2,
                        factor: 0.4
                    },
                    Effect::Gain { factor: 2.0 }
                ],
            }
        );
    }

    #[test]
    fn no_arg_returns_missing_input() {
        let args: Vec<String> = vec![];

        assert_eq!(parse_args(&args).unwrap_err(), ParseError::MissingInput);
    }

    #[test]
    fn one_arg_returns_missing_output() {
        let args: Vec<String> = vec!["in.wav".into()];

        assert_eq!(parse_args(&args).unwrap_err(), ParseError::MissingOutput);
    }

    #[test]
    fn unknown_effect_returns_unknown_effect() {
        let args: Vec<String> = vec!["in.wav".into(), "out.wav".into(), "not-an-effect".into()];

        assert_eq!(
            parse_args(&args).unwrap_err(),
            ParseError::UnknownEffect("not-an-effect".into())
        );
    }

    #[test]
    fn gain_without_param_returns_missing_param() {
        let args: Vec<String> = vec!["in.wav".into(), "out.wav".into(), "gain".into()];

        assert_eq!(
            parse_args(&args).unwrap_err(),
            ParseError::MissingParam("gain".into())
        );
    }

    #[test]
    fn echo_with_one_param_returns_missing_param() {
        let args: Vec<String> = vec!["in.wav".into(), "out.wav".into(), "echo".into(), "2".into()];

        assert_eq!(
            parse_args(&args).unwrap_err(),
            ParseError::MissingParam("echo".into())
        );
    }

    #[test]
    fn lowpass_without_param_returns_missing_param() {
        let args: Vec<String> = vec!["in.wav".into(), "out.wav".into(), "lowpass".into()];

        assert_eq!(
            parse_args(&args).unwrap_err(),
            ParseError::MissingParam("lowpass".into())
        );
    }

    #[test]
    fn gain_with_non_numeric_param_returns_invalid_number() {
        let args: Vec<String> = vec![
            "in.wav".into(),
            "out.wav".into(),
            "gain".into(),
            "not-a-number".into(),
            "0.4".into(),
        ];

        assert_eq!(
            parse_args(&args).unwrap_err(),
            ParseError::InvalidNumber("not-a-number".into())
        );
    }

    #[test]
    fn out_of_domain_numeric_param_returns_invalid_number() {
        let args: Vec<String> = vec![
            "in.wav".into(),
            "out.wav".into(),
            "echo".into(),
            "1.5".into(),
            "0.4".into(),
        ];

        assert_eq!(
            parse_args(&args).unwrap_err(),
            ParseError::InvalidNumber("1.5".into())
        );
    }
}
