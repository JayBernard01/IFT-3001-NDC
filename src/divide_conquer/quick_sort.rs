use crate::tests::partition::partition;

/// Best algorithm overall with randomized partition
pub fn quick_sort(vector: &mut [usize]) {
    if vector.len() <= 1 {
        return;
    }

    let pivot_index = partition(vector);
    quick_sort(&mut vector[..pivot_index]);
    quick_sort(&mut vector[pivot_index + 1..]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{divide_conquer::merge_sort::merge_sort, tests::fake_data};

    #[test]
    fn given_random_odd_elements_distribution_when_quick_sort_then_quick_sort() {
        let given_size = 5;
        let mut that_vector = fake_data::new_vector_with_random_distribution(given_size);

        quick_sort(&mut that_vector.as_mut_slice());

        assert!(that_vector.is_sorted());
    }

    #[test]
    fn given_sorted_distribution_when_quick_sort_then_quick_sort() {
        let given_size = 5;
        let that_vector = fake_data::new_vector_with_random_distribution(given_size);
        let mut that_sorted_vector = merge_sort(that_vector);

        quick_sort(&mut that_sorted_vector.as_mut_slice());

        assert!(that_sorted_vector.is_sorted());
    }

    #[test]
    fn given_random_even_elements_distribution_when_quick_sort_then_quick_sort() {
        let given_size = 6;
        let mut that_vector = fake_data::new_vector_with_random_distribution(given_size);

        quick_sort(&mut that_vector.as_mut_slice());

        assert!(that_vector.is_sorted());
    }

    #[test]
    fn given_no_elements_distribution_when_quick_sort_then_quick_sort() {
        let given_size = 0;
        let mut that_vector = fake_data::new_vector_with_random_distribution(given_size);

        quick_sort(&mut that_vector.as_mut_slice());

        assert!(that_vector.is_sorted());
    }

    #[test]
    fn given_one_element_distribution_when_quick_sort_then_quick_sort() {
        let given_size = 1;
        let mut that_vector = fake_data::new_vector_with_random_distribution(given_size);

        quick_sort(&mut that_vector.as_mut_slice());

        assert!(that_vector.is_sorted());
    }
}
