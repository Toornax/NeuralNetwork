use self::numerical::Number;
use std::ops::{Add, Sub, Mul, Div};

struct Matrix<T: Number> {
    size: (usize, usize)
    coefs: Vec<T>
}

impl<T: Number> Matrix<T> {
    pub fn new(nb_row: usize, nb_col: usize) -> Self {
        Matrix<T> {
            size: (nb_row, nb_col),
            coefs: Vec::new()
        }
    }
}

impl<T: Number> Add for Matrix<T> {
    type Output = Self;

    fn add(self, other: Matrix<T>) -> Self {
        
    }
}
