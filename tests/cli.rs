use audiodsp::cli;
use audiodsp::dsp;
use audiodsp::io;
use std::path::Path;

#[test]
fn full_pipeline_processes_fixture_sample_accurately() {
    let expected: Vec<f32> = vec![0.0, 0.5, -0.25, -0.875];

    let input = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/reference.wav"
    ));

    let dir = tempfile::tempdir().expect("should create a temp dir");
    let output = dir.path().join("out.wav");

    let args: Vec<String> = vec![
        input.to_str().unwrap().into(),
        output.to_str().unwrap().into(),
        "gain".into(),
        "2".into(),
        "lowpass".into(),
        "0.5".into(),
        "echo".into(),
        "2".into(),
        "0.5".into(),
    ];

    let config = cli::parse_args(&args).unwrap();

    let (samples, sample_rate) = io::read_wav(Path::new(&config.input)).unwrap();

    let result = dsp::apply_chain(&config.effects, &samples);

    assert_eq!(
        result.len(),
        expected.len(),
        "must preserve the sample count"
    );

    for (&result, expected) in result.iter().zip(expected) {
        assert!(
            (result - expected).abs() < 1e-6,
            "sample differs: wanted {expected}, got {result}",
        );
    }

    io::write_wav(Path::new(&config.output), &result, sample_rate).unwrap();
}
