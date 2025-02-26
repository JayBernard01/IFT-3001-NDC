use rand::Rng;

const MAX_VALUE: usize = 100;

pub fn new_vector_with_random_distribution(size: usize) -> Vec<usize> {
    let mut rng = rand::rng();
    (0..size).map(|_| rng.random_range(0..MAX_VALUE)).collect()
}

pub fn new_number_not_in_distribution() -> usize {
    return MAX_VALUE + 500;
}
