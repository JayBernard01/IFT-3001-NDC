use rand::Rng;

pub fn new_vector_with_random_distribution(size: usize) -> Vec<usize> {
    let mut rng = rand::rng();
    (0..size).map(|_| rng.random_range(0..100)).collect()
}
