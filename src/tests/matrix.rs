use std::fmt;

///Squared Matrix
#[derive(Debug, Clone)]
pub struct Matrix {
    pub size: usize,
    pub data: Vec<Vec<i32>>,
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.data {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}
impl Matrix {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        Self {
            size,
            data: vec![vec![0; size]; size],
        }
    }

    pub fn new_matrix_with_data(data: Vec<i32>) -> Self {
        assert!(data.len() > 0);
        let size = (data.len() as f64).sqrt() as usize;
        let mut matrix = Matrix::new(size);
        for i in 0..size {
            for j in 0..size {
                matrix.data[i][j] = data[i * size + j];
            }
        }
        matrix
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn is_equals_size_to(&self, other: &Matrix) -> bool {
        self.size == other.size
    }

    pub fn is_size_even(&self) -> bool {
        self.size % 2 == 0
    }

    pub fn at(&self, row: usize, col: usize) -> Option<i32> {
        if row < self.size && col < self.size {
            Some(self.data[row][col])
        } else {
            None
        }
    }

    pub fn add(&self, other: &Matrix) -> Matrix {
        assert!(self.is_equals_size_to(other));
        let mut result = Matrix::new(self.size);
        for i in 0..self.size {
            for j in 0..self.size {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }

    pub fn subtract(&self, other: &Matrix) -> Matrix {
        assert!(self.is_equals_size_to(other));
        let mut result = Matrix::new(self.size);
        for i in 0..self.size {
            for j in 0..self.size {
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        result
    }

    pub fn split(&self) -> (Matrix, Matrix, Matrix, Matrix) {
        assert!(self.get_size() > 1);
        let new_size = self.size / 2;
        let mut a00 = Matrix::new(new_size);
        let mut a01 = Matrix::new(new_size);
        let mut a10 = Matrix::new(new_size);
        let mut a11 = Matrix::new(new_size);

        for i in 0..new_size {
            for j in 0..new_size {
                a00.data[i][j] = self.data[i][j];
                a01.data[i][j] = self.data[i][j + new_size];
                a10.data[i][j] = self.data[i + new_size][j];
                a11.data[i][j] = self.data[i + new_size][j + new_size];
            }
        }
        (a00, a01, a10, a11)
    }

    pub fn merge(c00: &Matrix, c01: &Matrix, c10: &Matrix, c11: &Matrix) -> Matrix {
        assert!(c00.is_equals_size_to(c01));
        assert!(c00.is_equals_size_to(c10));
        assert!(c00.is_equals_size_to(c11));

        let new_size = c00.size * 2;
        let mut result = Matrix::new(new_size);

        for i in 0..c00.size {
            for j in 0..c00.size {
                result.data[i][j] = c00.data[i][j];
                result.data[i][j + c00.size] = c01.data[i][j];
                result.data[i + c00.size][j] = c10.data[i][j];
                result.data[i + c00.size][j + c00.size] = c11.data[i][j];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_size_one_when_new_matrix_then_return_filled_zero_matrix_one_zero() {
        let matrix = Matrix::new(1);

        assert_eq!(matrix.get_size(), 1);
        assert_eq!(matrix.data, vec![vec![0]]);
    }

    #[test]
    fn given_size_more_than_one_even_when_new_matrix_then_return_filled_zero_matrix_two_zero() {
        let matrix = Matrix::new(2);

        assert_eq!(matrix.get_size(), 2);
        assert_eq!(matrix.data, vec![vec![0, 0], vec![0, 0]]);
    }

    #[test]
    fn given_size_one_with_data_when_new_matrix_with_data_then_return_matrix_with_that_data() {
        let matrix = Matrix::new_matrix_with_data(vec![5]);

        assert_eq!(matrix.get_size(), 1);
        assert_eq!(matrix.data, vec![vec![5]]);
    }

    #[test]
    fn given_size_more_than_one_even_data_when_new_matrix_with_data_then_return_matrix_with_that_data_column_then_line(
    ) {
        let matrix = Matrix::new_matrix_with_data(vec![1, 2, 3, 4]);

        assert_eq!(matrix.get_size(), 2);
        assert_eq!(matrix.data, vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn given_two_matrix_size_one_when_add_return_matrix_size_one_with_sum_value() {
        let a = Matrix::new_matrix_with_data(vec![3]);
        let b = Matrix::new_matrix_with_data(vec![7]);

        let result = a.add(&b);

        assert_eq!(result.data, vec![vec![10]]);
    }

    #[test]
    fn given_two_matrix_size_more_than_one_even_when_add_return_matrix_size_one_with_sum_value_for_each_same_coordinates_elements_other_matrix(
    ) {
        let a = Matrix::new_matrix_with_data(vec![1, 2, 3, 4]);
        let b = Matrix::new_matrix_with_data(vec![5, 6, 7, 8]);

        let result = a.add(&b);

        assert_eq!(result.data, vec![vec![6, 8], vec![10, 12]]);
    }

    #[test]
    fn given_two_matrix_size_one_when_subtract_return_matrix_size_one_with_subtract_value() {
        let a = Matrix::new_matrix_with_data(vec![8]);
        let b = Matrix::new_matrix_with_data(vec![3]);

        let result = a.subtract(&b);

        assert_eq!(result.data, vec![vec![5]]);
    }

    #[test]
    fn given_two_matrix_size_more_than_one_even_when_subtract_return_matrix_size_one_with_subtract_value_for_each_same_coordinates_elements_other_matrix(
    ) {
        let a = Matrix::new_matrix_with_data(vec![9, 7, 6, 5]);
        let b = Matrix::new_matrix_with_data(vec![4, 3, 2, 1]);

        let result = a.subtract(&b);

        assert_eq!(result.data, vec![vec![5, 4], vec![4, 4]]);
    }

    #[test]
    fn given_two_matrix_size_more_than_one_even_when_split_return_matrix_size_one_with_subtract_value_for_each_same_coordinates_elements_other_matrix(
    ) {
        let matrix = Matrix::new_matrix_with_data(vec![1, 2, 3, 4]);

        let (a00, a01, a10, a11) = matrix.split();

        assert_eq!(a00.data, vec![vec![1]]);
        assert_eq!(a01.data, vec![vec![2]]);
        assert_eq!(a10.data, vec![vec![3]]);
        assert_eq!(a11.data, vec![vec![4]]);
    }

    #[test]
    fn test_split_size_4() {
        let matrix = Matrix::new_matrix_with_data(vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        ]);

        let (a00, a01, a10, a11) = matrix.split();

        assert_eq!(a00.data, vec![vec![1, 2], vec![5, 6]]);
        assert_eq!(a01.data, vec![vec![3, 4], vec![7, 8]]);
        assert_eq!(a10.data, vec![vec![9, 10], vec![13, 14]]);
        assert_eq!(a11.data, vec![vec![11, 12], vec![15, 16]]);
    }

    #[test]
    fn test_merge_size_1() {
        let a00 = Matrix::new_matrix_with_data(vec![1]);
        let a01 = Matrix::new_matrix_with_data(vec![2]);
        let a10 = Matrix::new_matrix_with_data(vec![3]);
        let a11 = Matrix::new_matrix_with_data(vec![4]);

        let merged = Matrix::merge(&a00, &a01, &a10, &a11);

        assert_eq!(merged.data, vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn test_merge_size_2() {
        let a00 = Matrix::new_matrix_with_data(vec![1, 2, 5, 6]);
        let a01 = Matrix::new_matrix_with_data(vec![3, 4, 7, 8]);
        let a10 = Matrix::new_matrix_with_data(vec![9, 10, 13, 14]);
        let a11 = Matrix::new_matrix_with_data(vec![11, 12, 15, 16]);

        let merged = Matrix::merge(&a00, &a01, &a10, &a11);

        assert_eq!(
            merged.data,
            vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16]
            ]
        );
    }
}
