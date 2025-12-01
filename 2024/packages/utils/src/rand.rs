use std::hash::{BuildHasher, Hasher, RandomState};

pub fn shuffle<T>(vec: &mut [T]) {
    let n: usize = vec.len();
    for i in 0..(n - 1) {
        // Generate random index j, such that: i <= j < n
        // The remainder (`%`) after division is always less than the divisor.
        let j = (rand() as usize) % (n - i) + i;
        vec.swap(i, j);
    }
}

pub fn rand() -> u64 {
    RandomState::new().build_hasher().finish()
}
