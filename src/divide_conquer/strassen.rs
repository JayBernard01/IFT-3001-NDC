use crate::tests::matrix::Matrix;

/// We assume that the matrix_a is squared even and equal in size to matrix_b
pub fn strassen_multiplications(matrix_a: &Matrix, matrix_b: &Matrix) -> Matrix {
    assert!(matrix_a.is_equals_size_to(matrix_b));

    if matrix_a.get_size() == 1 {
        return Matrix::new_matrix_with_data(vec![matrix_a.data[0][0] * matrix_b.data[0][0]]);
    }

    assert!(matrix_a.is_size_even());
    let (a00, a01, a10, a11) = matrix_a.split();
    let (b00, b01, b10, b11) = matrix_b.split();

    let m1 = strassen_multiplications(&a00.add(&a11), &b00.add(&b11));
    let m2 = strassen_multiplications(&a10.add(&a11), &b00);
    let m3 = strassen_multiplications(&a00, &b01.subtract(&b11));
    let m4 = strassen_multiplications(&a11, &b10.subtract(&b00));
    let m5 = strassen_multiplications(&a00.add(&a01), &b11);
    let m6 = strassen_multiplications(&a10.subtract(&a00), &b00.add(&b01));
    let m7 = strassen_multiplications(&a01.subtract(&a11), &b10.add(&b11));

    let c00 = m1.add(&m4).subtract(&m5).add(&m7);
    let c01 = m3.add(&m5);
    let c10 = m2.add(&m4);
    let c11 = m1.add(&m3).subtract(&m2).add(&m6);

    Matrix::merge(&c00, &c01, &c10, &c11)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_two_matrix_with_size_2_multiply_them() {
        let matrix_a = Matrix::new_matrix_with_data(vec![1, 2, 3, 4]);
        let matrix_b = Matrix::new_matrix_with_data(vec![5, 6, 7, 8]);

        let result = strassen_multiplications(&matrix_a, &matrix_b);
        println!("{}", result);

        // Expected matrix after multiplication:
        // [1*5 + 2*7, 1*6 + 2*8]
        // [3*5 + 4*7, 3*6 + 4*8]
        let expected_result = Matrix::new_matrix_with_data(vec![19, 22, 43, 50]);

        assert_eq!(result.get_size(), 2);
        assert_eq!(result.data, expected_result.data);
    }

    #[test]
    fn given_two_matrix_with_size_4_multiply_them() {
        let matrix_a = Matrix::new_matrix_with_data(vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        ]);
        let matrix_b = Matrix::new_matrix_with_data(vec![
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        ]);

        let result = strassen_multiplications(&matrix_a, &matrix_b);
        println!("{}", result);

        // Expected matrix after multiplication:
        //     250  260  270  280
        //     618  644  670  696
        //     986 1028 1070 1112
        //    1354 1412 1470 1528
        let expected_result = Matrix::new_matrix_with_data(vec![
            250, 260, 270, 280, 618, 644, 670, 696, 986, 1028, 1070, 1112, 1354, 1412, 1470, 1528,
        ]);

        assert_eq!(result.get_size(), 4);
        assert_eq!(result.data, expected_result.data);
    }
}
