use std::ops::Mul;

use ndarray::{arr2, iter::Indices, s, Array2, Axis, Ix};

use crate::{point::Point, tuple::Tuple, vector::Vector};

#[derive(Clone, PartialEq, Debug)]
pub struct Matrix {
    elements: Array2<f64>,
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
        if self.elements.dim() != (2, 2) {
            panic!("cannot find the determinant of a matrix that isnt 2x2")
        }

        let a = self.get(0, 0).unwrap();
        let b = self.get(0, 1).unwrap();
        let c = self.get(1, 0).unwrap();
        let d = self.get(1, 1).unwrap();

        a * d - b * c
    }

    /// sub_matrix removes the given row and column from the current matrix and returns a new matrix.
    pub fn sub_matrix(&mut self, row: usize, column: usize) {
        self.elements.remove_index(Axis(0), row);
        self.elements.remove_index(Axis(1), column);
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
        let mut a = Matrix::new(arr2(&[[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]));
        let sub_matrix = Matrix::new(arr2(&[[-3.0, 2.0], [0.0, 6.0]]));

        a.sub_matrix(0, 2);

        assert_eq!(a, sub_matrix);
    }
}
