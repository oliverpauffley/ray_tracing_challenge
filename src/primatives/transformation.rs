use ndarray::arr2;

use crate::{
    primatives::matrix::Matrix,
    primatives::point::Point,
    primatives::tuple::Tuple,
    primatives::vector::{cross, Vector},
};

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::new(arr2(&[
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ]))
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::new(arr2(&[
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]))
}

pub fn rotation_x(angle: f64) -> Matrix {
    let cos_r = angle.cos();
    let sin_r = angle.sin();

    Matrix::new(arr2(&[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, cos_r, -sin_r, 0.0],
        [0.0, sin_r, cos_r, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]))
}

pub fn rotation_y(angle: f64) -> Matrix {
    let cos_r = angle.cos();
    let sin_r = angle.sin();

    Matrix::new(arr2(&[
        [cos_r, 0.0, sin_r, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-sin_r, 0.0, cos_r, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]))
}

pub fn rotation_z(angle: f64) -> Matrix {
    let cos_r = angle.cos();
    let sin_r = angle.sin();

    Matrix::new(arr2(&[
        [cos_r, -sin_r, 0.0, 0.0],
        [sin_r, cos_r, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]))
}

pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix {
    Matrix::new(arr2(&[
        [1.0, x_y, x_z, 0.0],
        [y_x, 1.0, y_z, 0.0],
        [z_x, z_y, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]))
}

pub fn view_transformation(from: Point, to: Point, up: Vector) -> Matrix {
    let forward = (to - from).norm();
    let left = cross(forward, up.norm());
    let true_up = cross(left, forward);

    let orientation = Matrix::new(arr2(&[
        [left.x(), left.y(), left.z(), 0.],
        [true_up.x(), true_up.y(), true_up.z(), 0.],
        [-forward.x(), -forward.y(), -forward.z(), 0.],
        [0., 0., 0., 1.],
    ]));
    orientation * translation(-from.x(), -from.y(), -from.z())
}

#[cfg(test)]
mod test_transformation {
    use std::f64::consts::PI;

    use crate::Tuple;
    use crate::P;
    use crate::V;

    use super::*;

    #[test]
    fn test_translation() {
        let translate = translation(5.0, -3.0, 2.0);
        let point = P![-3.0, 4.0, 5.0];
        let got = translate.clone() * point;
        assert_eq!(got, P![2.0, 1.0, 7.0]);

        let inverse = translate.inverse().unwrap();
        let got = inverse * point;
        assert_eq!(got, P![-8.0, 7.0, 3.0]);

        let vector = V![2.0, 1.0, 7.0];
        let got = translate * vector;
        // the vector should be unchanged by translation.
        assert_eq!(got, vector);
    }

    #[test]
    fn test_scaling() {
        let scale = scaling(2.0, 3.0, 4.0);

        let point = P![-4.0, 6.0, 8.0];
        let got = scale.clone() * point;
        assert_eq!(P![-8.0, 18.0, 32.0], got);

        let vector = V![-4.0, 6.0, 8.0];
        let got = scale.clone() * vector;
        assert_eq!(V![-8.0, 18.0, 32.0], got);

        let inverse = scale.inverse().unwrap();
        let got = inverse * vector;
        assert_eq!(V![-2.0, 2.0, 2.0], got);

        // scaling by a negative is reflection
        let scale = scaling(-1.0, 1.0, 1.0);
        let point = P![2.0, 3.0, 4.0];
        let got = scale * point;
        assert_eq!(P![-2.0, 3.0, 4.0], got);
    }

    #[test]
    fn test_rotation_x() {
        let point = P![0.0, 1.0, 0.0];
        let half_quarter = rotation_x(PI / 4.0);
        let quarter = rotation_x(PI / 2.0);

        let reverse_half_quarter = half_quarter.inverse().unwrap();

        let r_2_2 = 2.0_f64.sqrt() / 2.0;

        assert_eq!(P![0.0, r_2_2, r_2_2], half_quarter * point);
        assert_eq!(P![0.0, 0.0, 1.0], quarter * point);
        assert_eq!(P![0.0, r_2_2, -r_2_2], reverse_half_quarter * point);
    }

    #[test]
    fn test_rotation_y() {
        let point = P![0.0, 0.0, 1.0];
        let half_quarter = rotation_y(PI / 4.0);
        let quarter = rotation_y(PI / 2.0);

        let r_2_2 = 2.0_f64.sqrt() / 2.0;

        assert_eq!(P![r_2_2, 0.0, r_2_2], half_quarter * point);
        assert_eq!(P![1.0, 0.0, 0.0], quarter * point);
    }

    #[test]
    fn test_rotation_z() {
        let point = P![0.0, 1.0, 0.0];
        let half_quarter = rotation_z(PI / 4.0);
        let quarter = rotation_z(PI / 2.0);

        let r_2_2 = 2.0_f64.sqrt() / 2.0;

        assert_eq!(P![-r_2_2, r_2_2, 0.0], half_quarter * point);
        assert_eq!(P![-1.0, 0.0, 0.0], quarter * point);
    }

    #[test]
    fn test_shearing() {
        let point = P![2.0, 3.0, 4.0];
        let got = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * point;
        assert_eq!(P![5.0, 3.0, 4.0], got);

        let got = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0) * point;
        assert_eq!(P![6.0, 3.0, 4.0], got);

        let got = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0) * point;
        assert_eq!(P![2.0, 5.0, 4.0], got);

        let got = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0) * point;
        assert_eq!(P![2.0, 7.0, 4.0], got);

        let got = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0) * point;
        assert_eq!(P![2.0, 3.0, 6.0], got);

        let got = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0) * point;
        assert_eq!(P![2.0, 3.0, 7.0], got);
    }

    #[test]
    fn test_chaining() {
        let point = P![1.0, 0.0, 1.0];
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        // apply the transformations in order
        let p = a * point;
        let p = b * p;
        let p = c * p;

        assert_eq!(P![15.0, 0.0, 7.0], p);

        // apply all the transformations at the same time
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let got = c * b * a * point;
        assert_eq!(P![15.0, 0.0, 7.0], got);

        // apply all the transformations using a helper to avoid having to reverse the orders.
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let got = point.transform(&[a, b, c]);
        assert_eq!(P![15.0, 0.0, 7.0], got);
    }

    #[test]
    fn test_view_transformation() {
        // no transformation required
        let from = P![0., 0., 0.];
        let to = P![0., 0., -1.];
        let up = V![0., 1., 0.];

        let t = view_transformation(from, to, up);

        assert_eq!(t, Matrix::identity_matrix());

        // looking in the postive z direction is like looking in a mirror
        // so reflect using a negative scaling.
        let from = P![0., 0., 0.];
        let to = P![0., 0., 1.];
        let up = V![0., 1., 0.];

        let t = view_transformation(from, to, up);

        assert_eq!(t, scaling(-1., 1., -1.));

        // the view transformation is really moving the world
        let from = P![0., 0., 8.];
        let to = P![0., 0., 0.];
        let up = V![0., 1., 0.];

        let t = view_transformation(from, to, up);

        assert_eq!(t, translation(0., 0., -8.));

        // an arbitrary view
        let from = P![1., 3., 2.];
        let to = P![4., -2., 8.];
        let up = V![1., 1., 0.];

        let t = view_transformation(from, to, up);
        let want = Matrix::new(arr2(&[
            [
                -0.5070925528371099,
                0.5070925528371099,
                0.6761234037828132,
                -2.366431913239846,
            ],
            [
                0.7677159338596801,
                0.6060915267313263,
                0.12121830534626524,
                -2.8284271247461894,
            ],
            [
                -0.35856858280031806,
                0.5976143046671968,
                -0.7171371656006361,
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ]));

        assert_eq!(want, t);
    }
}
