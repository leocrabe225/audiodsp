#[track_caller]
pub(super) fn assert_close(got: &[f32], want: &[f32]) {
    assert_eq!(got.len(), want.len(), "must preserve the sample count");

    for (&got, &want) in got.iter().zip(want) {
        assert!(
            (got - want).abs() < 1e-6,
            "sample differs: wanted {want}, got {got}",
        );
    }
}
