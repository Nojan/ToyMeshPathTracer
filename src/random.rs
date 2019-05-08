fn xor_shift_32(val: u32) -> u32 {
    let mut x = val;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 15;
    return x;
}

pub fn random_float01(state: &mut u32) -> f32 {
    *state = xor_shift_32(*state);
    ((*state & 0xFFFFFF) as f32) / 16777216.0f32
}

#[cfg(test)]
mod tests {
    #[test]
    fn random_float01() {
        let mut state = 0xFA59B021u32;
        for _ in 0..10 {
            let rand_float = super::random_float01(&mut state);
            assert!(0.0 <= rand_float);
            assert!(1.0 >= rand_float);
        }
    }

    #[test]
    #[ignore]
    fn random_float01_full_range() {
        for mut state in u32::min_value()..=u32::max_value() {
            let rand_float = super::random_float01(&mut state);
            assert!(0.0 <= rand_float);
            assert!(1.0 >= rand_float);
        }
    }
}
