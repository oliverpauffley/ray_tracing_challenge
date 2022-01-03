use ndarray::arr2;

use crate::matrix::Matrix;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::new(arr2(&[
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ]))
}

#[cfg(test)]
mod test_transformation {
    use crate::Tuple;
    use crate::P;
    use crate::V;

    use super::*;

    #[test]
    fn test_multiply_translation_matrix() {
        let translate = translation(5.0, -3.0, 2.0);
        let point = P![-3.0, 4.0, 5.0];
        let got = translate.clone() * point;
        assert_eq!(got, P![2.0, 1.0, 7.0]);

        let inverse = translate.clone().inverse().unwrap();
        let got = inverse * point;
        assert_eq!(got, P![-8.0, 7.0, 3.0]);

        let vector = V![2.0, 1.0, 7.0];
        let got = translate * vector;
        // the vector should be unchanged by translation.
        assert_eq!(got, vector);
    }
}
