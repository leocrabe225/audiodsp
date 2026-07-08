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

fn main() {
    println!("Hello, world!");
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
    fn i16_array_returns_matching_f32_vector() {
        let converted = i16_array_to_f32_vector(&[0, 16384, i16::MIN]);
        assert_eq!(converted, vec![0.0, 0.5, -1.0]);
    }
}
