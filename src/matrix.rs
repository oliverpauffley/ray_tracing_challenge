use std::{fmt::Display, ops::Mul};

use ndarray::{arr2, Array2, Axis};

use crate::{comparison::approx_eq, point::Point, tuple::Tuple, vector::Vector};

#[derive(Clone, Debug)]
pub struct Matrix {
    elements: Array2<f64>,
}

#[derive(Debug, Clone)]
pub struct InversionError;

impl Display for InversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cannot invert this matrix")
    }
}

impl Matrix {
    pub fn new(elements: Array2<f64>) -> Self {
        Self { elements }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&f64> {
        self.elements.get((x, y))
    }

    pub fn indentity_matrix() -> Matrix {
        Matrix {
            elements: arr2(&[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
        }
    }

    pub fn transpose(&self) -> Matrix {
        let elements = self.elements.clone().reversed_axes();
        Matrix::new(elements)
    }

    pub fn determinant(&self) -> f64 {
        if self.elements.dim() == (2, 2) {
            let a = self.get(0, 0).unwrap();
            let b = self.get(0, 1).unwrap();
            let c = self.get(1, 0).unwrap();
            let d = self.get(1, 1).unwrap();
            a * d - b * c
        } else {
            // find the cofactors of the first row for non 2x2 matrices.
            self.elements
                .row(0)
                .indexed_iter()
                .fold(0.0, |deter, x| deter + (x.1 * self.cofactor(0, x.0)))
        }
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    /// sub_matrix removes the given row and column from the current matrix and returns a new matrix.
    pub fn sub_matrix(&self, row: usize, column: usize) -> Matrix {
        let mut working_matrix = self.clone();
        working_matrix.elements.remove_index(Axis(0), row);
        working_matrix.elements.remove_index(Axis(1), column);
        working_matrix
    }

    /// minor finds the determinant of the submatrix at the given position
    pub fn minor(&self, row: usize, column: usize) -> f64 {
        let matrix = self.sub_matrix(row, column);
        matrix.determinant()
    }

    /// cofactor finds the minor of a matrix at a given position but flips the sign if the row index + column index is an odd number.
    pub fn cofactor(&self, row: usize, column: usize) -> f64 {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn inverse(&self) -> Result<Matrix, InversionError> {
        if !self.is_invertible() {
            return Err(InversionError);
        }

        let determinant = self.determinant();
        let mut m = self.clone();

        self.elements.indexed_iter().for_each(|(index, _)| {
            let c = self.cofactor(index.0, index.1);
            m.elements[[index.1, index.0]] = c / determinant;
        });
        Ok(m)
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Matrix::indentity_matrix()
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let elements = self.elements.dot(&rhs.elements);
        Matrix::new(elements)
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let vec = self
            .elements
            .dot(&arr2(&[[rhs.x()], [rhs.y()], [rhs.z()], [rhs.w()]]));
        Point::new(
            *vec.get((0, 0)).unwrap(),
            *vec.get((1, 0)).unwrap(),
            *vec.get((2, 0)).unwrap(),
        )
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let vec = self
            .elements
            .dot(&arr2(&[[rhs.x()], [rhs.y()], [rhs.z()], [rhs.w()]]));
        Vector::new(
            *vec.get((0, 0)).unwrap(),
            *vec.get((1, 0)).unwrap(),
            *vec.get((2, 0)).unwrap(),
        )
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.elements
            .iter()
            .zip(other.elements.iter())
            .all(|(a, b)| approx_eq(*a, *b))
    }
}

#[cfg(test)]
mod test_matrix {
    use ndarray::arr2;

    use crate::{comparison::approx_eq, tuple::Tuple, P, V};

    use super::*;

    #[test]
    fn test_new() {
        let elements = arr2(&[
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        let matrix = Matrix::new(elements);
        assert!(approx_eq(1.0, *matrix.get(0, 0).unwrap()));
        assert!(approx_eq(4.0, *matrix.get(0, 3).unwrap()));
        assert!(approx_eq(5.5, *matrix.get(1, 0).unwrap()));
        assert!(approx_eq(7.5, *matrix.get(1, 2).unwrap()));
        assert!(approx_eq(11.0, *matrix.get(2, 2).unwrap()));
        assert!(approx_eq(13.5, *matrix.get(3, 0).unwrap()));
        assert!(approx_eq(15.5, *matrix.get(3, 2).unwrap()));
    }

    #[test]
    fn test_2x2() {
        let elements = arr2(&[[-3.0, 5.0], [1.0, -2.0]]);
        let matrix = Matrix::new(elements);
        assert!(approx_eq(-3.0, *matrix.get(0, 0).unwrap()));
        assert!(approx_eq(5.0, *matrix.get(0, 1).unwrap()));
        assert!(approx_eq(1.0, *matrix.get(1, 0).unwrap()));
        assert!(approx_eq(-2.0, *matrix.get(1, 1).unwrap()));
    }

    #[test]
    fn test_3x3() {
        let elements = arr2(&[[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);
        let matrix = Matrix::new(elements);
        assert!(approx_eq(-3.0, *matrix.get(0, 0).unwrap()));
        assert!(approx_eq(-2.0, *matrix.get(1, 1).unwrap()));
        assert!(approx_eq(1.0, *matrix.get(2, 2).unwrap()));
    }

    #[test]
    fn test_equal() {
        let elements = arr2(&[[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);
        let matrix_1 = Matrix::new(elements.clone());
        let matrix_2 = Matrix::new(elements);
        assert_eq!(matrix_1, matrix_2);

        let elements = arr2(&[[-3.0, 5.0], [1.0, -2.0]]);
        let matrix_1 = Matrix::new(elements.clone());
        let matrix_2 = Matrix::new(elements);
        assert_eq!(matrix_1, matrix_2);
    }
    #[test]
    fn test_multiply() {
        let elements = arr2(&[
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let a = Matrix::new(elements);
        let elements = arr2(&[
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let b = Matrix::new(elements);

        let res = a * b;

        assert_eq!(
            res.elements,
            arr2(&[
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0],
            ])
        )
    }

    #[test]
    fn test_multiply_tuple() {
        let elements = arr2(&[
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let matrix = Matrix::new(elements.clone());

        let point = P![1.0, 2.0, 3.0];
        let res = matrix * point;
        assert_eq!(P![18.0, 24.0, 33.0], res);

        let matrix = Matrix::new(elements);
        let vector = V![1.0, 2.0, 3.0];
        let res = matrix * vector;
        assert_eq!(V![14.0, 22.0, 32.0], res);
    }

    #[test]
    fn test_multiply_identity() {
        let elements = arr2(&[
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let a = Matrix::new(elements);
        let res = a.clone() * Matrix::indentity_matrix();

        assert_eq!(res, a)
    }

    #[test]
    fn test_transpose() {
        let a = Matrix::new(arr2(&[
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]));
        let a_tranpose = Matrix::new(arr2(&[
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]));

        assert_eq!(a.transpose(), a_tranpose);
    }

    #[test]
    fn test_determinant() {
        let a = Matrix::new(arr2(&[[1.0, 5.0], [-3.0, 2.0]]));
        assert!(approx_eq(17.0, a.determinant()))
    }
    #[test]
    fn test_submatrix() {
        let a = Matrix::new(arr2(&[[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]));
        let sub_matrix = Matrix::new(arr2(&[[-3.0, 2.0], [0.0, 6.0]]));

        let got = a.sub_matrix(0, 2);

        assert_eq!(got, sub_matrix);

        let b = Matrix::new(arr2(&[
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]));

        let sub_matrix = Matrix::new(arr2(&[
            [-6.0, 1.0, 6.0],
            [-8.0, 8.0, 6.0],
            [-7.0, -1.0, 1.0],
        ]));

        let got = b.sub_matrix(2, 1);

        assert_eq!(got, sub_matrix)
    }

    #[test]
    fn test_minors() {
        let a = Matrix::new(arr2(&[
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0],
        ]));

        let sub_matrix = a.sub_matrix(1, 0);

        assert!(approx_eq(sub_matrix.determinant(), 25.0));
        assert!(approx_eq(a.minor(1, 0), 25.0));
    }

    #[test]
    fn test_cofactors() {
        let a = Matrix::new(arr2(&[
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0],
        ]));
        assert!(approx_eq(a.minor(0, 0), -12.0));
        assert!(approx_eq(a.cofactor(0, 0), -12.0));
        assert!(approx_eq(a.minor(1, 0), 25.0));
        assert!(approx_eq(a.cofactor(1, 0), -25.0));
    }

    #[test]
    fn test_determinant_3x3() {
        let a = Matrix::new(arr2(&[[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]));
        assert!(approx_eq(a.cofactor(0, 0), 56.0));
        assert!(approx_eq(a.cofactor(0, 1), 12.0));
        assert!(approx_eq(a.cofactor(0, 2), -46.0));
        assert!(approx_eq(a.determinant(), -196.0));
    }

    #[test]
    fn test_determinant_4x4() {
        let a = Matrix::new(arr2(&[
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]));
        assert!(approx_eq(a.cofactor(0, 0), 690.0));
        assert!(approx_eq(a.cofactor(0, 1), 447.0));
        assert!(approx_eq(a.cofactor(0, 2), 210.0));
        assert!(approx_eq(a.cofactor(0, 3), 51.0));
        assert!(approx_eq(a.determinant(), -4071.0));
    }

    #[test]
    fn test_is_invertible() {
        let a = Matrix::new(arr2(&[
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]));
        assert!(approx_eq(a.determinant(), -2120.0));
        assert!(a.is_invertible());

        let b = Matrix::new(arr2(&[
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]));
        assert!(approx_eq(b.determinant(), 0.0));
        assert!(!b.is_invertible());
    }

    #[test]
    fn test_inverse() {
        let a = Matrix::new(arr2(&[
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]));
        let b = a.inverse().unwrap();

        assert!(approx_eq(a.determinant(), 532.0));
        assert!(approx_eq(a.cofactor(2, 3), -160.0));
        assert!(approx_eq(*b.get(3, 2).unwrap(), -160.0 / 532.0));
        assert!(approx_eq(a.cofactor(3, 2), 105.0));
        assert!(approx_eq(*b.get(2, 3).unwrap(), 105.0 / 532.0));

        let want = Matrix::new(arr2(&[
            [0.218051, 0.451127, 0.240601, -0.045112],
            [-0.808270, -1.456766, -0.443609, 0.520676],
            [-0.078947, -0.223684, -0.052631, 0.197368],
            [-0.522556, -0.813909, -0.300751, 0.3063909],
        ]));

        assert_eq!(want, b);

        let a = Matrix::new(arr2(&[
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]));

        let want = Matrix::new(arr2(&[
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]));

        assert_eq!(want, a.inverse().unwrap());

        let a = Matrix::new(arr2(&[
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]));

        let want = Matrix::new(arr2(&[
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]));

        assert_eq!(want, a.inverse().unwrap());
    }

    #[test]
    fn test_inverser_reverse_product() {
        let a = Matrix::new(arr2(&[
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]));

        let b = Matrix::new(arr2(&[
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]));

        let c = a.clone() * b.clone();
        assert_eq!(a, c * b.inverse().unwrap());
    }
}
