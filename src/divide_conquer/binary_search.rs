use std::usize;

/// We assume that the vector is already sorted, because we would otherwise iterate over all elements.
/// Useful for pre-sorted static distribution.
pub fn binary_search(vector: Vec<usize>, value: usize) -> Option<usize> {
    let vector_size = vector.len();
    if vector_size == 0 {
        return None;
    }

    let mut left = 0;
    let mut right = vector_size - 1;
    let mut mid;

    while left <= right {
        mid = (left + right) / 2;
        let pointed_value = vector[mid];
        if pointed_value == value {
            return Some(value);
        }
        if value < pointed_value {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{divide_conquer::merge_sort::merge_sort, tests::fake_data};

    #[test]
    fn given_random_odd_distribution_of_elements_with_element_middle_distribution_when_binary_search_return_element(
    ) {
        let given_size = 5;
        let mid_position = 3;
        let that_vector = fake_data::new_vector_with_random_distribution(given_size);
        let that_sorted_vector = merge_sort(that_vector);
        let expected_element = that_sorted_vector[mid_position];
        println!("{:?}", that_sorted_vector);

        let actual_element = binary_search(that_sorted_vector, expected_element);

        assert_eq!(Some(expected_element), actual_element);
    }

    #[test]
    fn given_random_even_distribution_of_elements_with_element_start_distribution_when_binary_search_return_element(
    ) {
        let given_size = 6;
        let mid_position = 0;
        let that_vector = fake_data::new_vector_with_random_distribution(given_size);
        let that_sorted_vector = merge_sort(that_vector);
        let expected_element = that_sorted_vector[mid_position];
        println!("{:?}", that_sorted_vector);

        let actual_element = binary_search(that_sorted_vector, expected_element);

        assert_eq!(Some(expected_element), actual_element);
    }

    #[test]
    fn given_random_even_distribution_of_elements_with_element_end_distribution_when_binary_search_return_element(
    ) {
        let given_size = 6;
        let mid_position = 5;
        let that_vector = fake_data::new_vector_with_random_distribution(given_size);
        let that_sorted_vector = merge_sort(that_vector);
        let expected_element = that_sorted_vector[mid_position];
        println!("{:?}", that_sorted_vector);

        let actual_element = binary_search(that_sorted_vector, expected_element);

        assert_eq!(Some(expected_element), actual_element);
    }

    #[test]
    fn given_random_odd_number_of_elements_with_element_not_in_distribution_when_binary_search_return_none(
    ) {
        let given_size = 5;
        let that_vector = fake_data::new_vector_with_random_distribution(given_size);
        let that_sorted_vector = merge_sort(that_vector);

        let actual_element = binary_search(
            that_sorted_vector,
            fake_data::new_number_not_in_distribution(),
        );

        assert_eq!(None, actual_element);
    }

    #[test]
    fn given_no_elements_in_distribution_when_binary_search_return_none() {
        let given_size = 0;
        let that_vector = fake_data::new_vector_with_random_distribution(given_size);
        let that_sorted_vector = merge_sort(that_vector);

        let actual_element = binary_search(
            that_sorted_vector,
            fake_data::new_number_not_in_distribution(),
        );

        assert_eq!(None, actual_element);
    }

    #[test]
    fn given_random_even_number_of_elements_in_distribution_with_element_not_in_distribution_when_binary_search_return_none(
    ) {
        let given_size = 6;
        let that_vector = fake_data::new_vector_with_random_distribution(given_size);
        let that_sorted_vector = merge_sort(that_vector);

        let actual_element = binary_search(
            that_sorted_vector,
            fake_data::new_number_not_in_distribution(),
        );

        assert_eq!(None, actual_element);
    }
}
