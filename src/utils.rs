use std::time::{SystemTime, UNIX_EPOCH};

use rand::XorShiftRng;

pub mod convert;
mod rand;

pub fn shuffle_vec<T>(arr: &mut [T]) {
    let seed = time_now();

    let len = arr.len();
    let mut rng = XorShiftRng::new(seed);

    for i in (1..len).rev() {
        let j = rng.gen_range(0..=i.try_into().expect("Cant convert element of array!"));
        arr.swap(i, j.try_into().expect("Cant convert element of array!"));
    }
}

pub fn time_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Clockwork may gone backward")
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle_1() {
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
        let copy_arr = arr.clone();

        shuffle_vec(&mut arr);
        dbg!(&arr);

        assert_ne!(&arr, &copy_arr);

        arr.sort();

        assert_eq!(&arr, &copy_arr);
    }

    #[test]
    fn test_shuffle_2() {
        let mut arr = vec![-200, -193, 0, 1, 2, 3, 4, 5, 6, 7, 10000];
        let copy_arr = arr.clone();

        shuffle_vec(&mut arr);
        dbg!(&arr);

        assert_ne!(&arr, &copy_arr);

        arr.sort();

        assert_eq!(&arr, &copy_arr);
    }

    #[test]
    fn test_shuffle_3() {
        let mut arr: Vec<u32> = (1..=1_000_000).collect();
        let copy_arr = arr.clone();

        shuffle_vec(&mut arr);

        assert_ne!(&arr, &copy_arr);

        arr.sort();

        assert_eq!(&arr, &copy_arr);
    }
}
