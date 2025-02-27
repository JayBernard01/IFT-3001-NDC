/// Best algorithm for noobs who don't check if it's already sorted
pub fn insertion_sort(vector: &mut [usize]) {
    let vector_len = vector.len();
    for i in 1..vector_len {
        let key = vector[i];
        let mut j = i;

        while j > 0 && vector[j - 1] > key {
            vector[j] = vector[j - 1];
            j -= 1;
        }

        vector[j] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{divide_conquer::merge_sort::merge_sort, tests::fake_data};

    #[test]
    fn given_random_odd_elements_distribution_when_insertion_sort_then_insertion_sort() {
        let given_size = 5;
        let mut that_vector = fake_data::new_vector_with_random_distribution(given_size);

        insertion_sort(&mut that_vector.as_mut_slice());

        assert!(that_vector.is_sorted());
    }

    #[test]
    fn given_sorted_distribution_when_insertion_sort_then_insertion_sort() {
        let given_size = 5;
        let that_vector = fake_data::new_vector_with_random_distribution(given_size);
        let mut that_sorted_vector = merge_sort(that_vector);

        insertion_sort(&mut that_sorted_vector.as_mut_slice());

        assert!(that_sorted_vector.is_sorted());
    }

    #[test]
    fn given_random_even_elements_distribution_when_insertion_sort_then_insertion_sort() {
        let given_size = 6;
        let mut that_vector = fake_data::new_vector_with_random_distribution(given_size);

        insertion_sort(&mut that_vector.as_mut_slice());

        assert!(that_vector.is_sorted());
    }

    #[test]
    fn given_no_elements_distribution_when_insertion_sort_then_insertion_sort() {
        let given_size = 0;
        let mut that_vector = fake_data::new_vector_with_random_distribution(given_size);

        insertion_sort(&mut that_vector.as_mut_slice());

        assert!(that_vector.is_sorted());
    }

    #[test]
    fn given_one_element_distribution_when_insertion_sort_then_insertion_sort() {
        let given_size = 1;
        let mut that_vector = fake_data::new_vector_with_random_distribution(given_size);

        insertion_sort(&mut that_vector.as_mut_slice());

        assert!(that_vector.is_sorted());
    }
}
