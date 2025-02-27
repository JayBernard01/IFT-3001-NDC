use rand::Rng;

/// We assume random pivot in partition
pub fn partition(vector_part: &mut [usize]) -> usize {
    let partition_len = vector_part.len();
    if partition_len == 0 {
        return 0;
    }

    let pivot_idx = rand::rng().random_range(0..partition_len);
    let pivot = vector_part[pivot_idx];

    vector_part.swap(pivot_idx, partition_len - 1);
    let mut i = 0;

    for j in 0..partition_len - 1 {
        if vector_part[j] < pivot {
            vector_part.swap(i, j);
            i += 1;
        }
    }
    vector_part.swap(i, partition_len - 1);

    i
}
