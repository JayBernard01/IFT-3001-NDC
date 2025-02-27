pub fn merge(left: Vec<usize>, right: Vec<usize>) -> Vec<usize> {
    let mut temp = Vec::with_capacity(left.len() + right.len());
    let (mut left_iter, mut right_iter) = (left.into_iter(), right.into_iter());
    let (mut left_val, mut right_val) = (left_iter.next(), right_iter.next());

    while let (Some(l), Some(r)) = (left_val, right_val) {
        if l <= r {
            temp.push(l);
            left_val = left_iter.next();
        } else {
            temp.push(r);
            right_val = right_iter.next();
        }
    }

    // Append remaining elements without shifting
    temp.extend(left_val.into_iter().chain(left_iter));
    temp.extend(right_val.into_iter().chain(right_iter));

    temp
}

/// Useful when we want to guarantee theta(n log n) to sort, but requires more memory, not used in practice
pub fn merge_sort(mut vector: Vec<usize>) -> Vec<usize> {
    if vector.len() <= 1 {
        return vector;
    }

    let mid = vector.len() / 2;
    let right = vector.split_off(mid);
    let left = vector;

    merge(merge_sort(left), merge_sort(right))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::fake_data;

    #[test]
    fn given_random_odd_elements_distribution_when_merge_sort_then_merge_sort() {
        let given_size = 5;
        let that_vector: Vec<usize> = fake_data::new_vector_with_random_distribution(given_size);

        let sorted_vector = merge_sort(that_vector);

        assert!(sorted_vector.is_sorted());
    }

    #[test]
    fn given_random_even_elements_distribution_when_merge_sort_then_merge_sort() {
        let given_size = 6;
        let that_vector: Vec<usize> = fake_data::new_vector_with_random_distribution(given_size);

        let sorted_vector = merge_sort(that_vector);

        assert!(sorted_vector.is_sorted());
    }

    #[test]
    fn given_no_elements_distribution_when_merge_sort_then_merge_sort() {
        let given_size = 0;
        let that_vector: Vec<usize> = fake_data::new_vector_with_random_distribution(given_size);

        let sorted_vector = merge_sort(that_vector);

        assert!(sorted_vector.is_sorted());
    }

    #[test]
    fn given_one_element_distribution_when_merge_sort_then_merge_sort() {
        let given_size = 1;
        let that_vector: Vec<usize> = fake_data::new_vector_with_random_distribution(given_size);

        let sorted_vector = merge_sort(that_vector);

        assert!(sorted_vector.is_sorted());
    }
}
