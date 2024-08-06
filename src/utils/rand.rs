use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_random_number(max_number: usize) -> usize {
    // Get the current system time in nanoseconds since the UNIX epoch
    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as u64;

    // Use the nanoseconds value as a seed and generate a pseudo-random number
    seed = (seed ^ (seed >> 33)).wrapping_mul(0xff51afd7ed558ccd);
    seed = (seed ^ (seed >> 33)).wrapping_mul(0xc4ceb9fe1a85ec53);
    seed = seed ^ (seed >> 33);

    // Modulo operation to get a value between 0 and max_number - 1
    (seed % max_number as u64) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand_1() {
        let max_n = 40;
        let rn = get_random_number(max_n);
        dbg!(rn);

        assert!(rn < max_n);
    }

    #[test]
    fn test_rand_2() {
        let max_n = 231;
        let rn = get_random_number(max_n);
        dbg!(rn);

        assert!(rn < max_n);
    }
}
