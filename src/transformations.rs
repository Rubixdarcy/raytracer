use crate::prelude::*;

pub fn translation(x: f64, y: f64, z: f64) -> M4 {
    matrix4(1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,)
}

pub fn scaling(x: f64, y: f64, z: f64) -> M4 {
    matrix4(x,   0.0, 0.0, 0.0,
            0.0, y,   0.0, 0.0,
            0.0, 0.0, z,   0.0,
            0.0, 0.0, 0.0, 1.0,)
}

pub fn rotation_x(theta: f64) -> M4 {
    let (sin, cos) = theta.sin_cos();
    matrix4(1.0, 0.0, 0.0,  0.0,
            0.0, cos, -sin, 0.0,
            0.0, sin, cos,  0.0,
            0.0, 0.0, 0.0,  1.0)
}

pub fn rotation_y(theta: f64) -> M4 {
    let (sin, cos) = theta.sin_cos();
    matrix4(cos,  0.0, sin, 0.0,
            0.0,  1.0, 0.0, 0.0,
            -sin, 0.0, cos, 0.0,
            0.0,  0.0, 0.0, 1.0,)
}

pub fn rotation_z(theta: f64) -> M4 {
    let (sin, cos) = theta.sin_cos();
    matrix4(cos, -sin, 0.0, 0.0,
            sin, cos,  0.0, 0.0,
            0.0, 0.0,  1.0, 0.0,
            0.0, 0.0,  0.0, 1.0,)
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> M4 {
    matrix4(1.0, xy,  xz,  0.0,
            yx,  1.0, yz,  0.0,
            zx,  zy,  1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,)
}

pub fn view_transform(from: T4, to: T4, up: T4) -> M4 {
    let forward = (to - from).normalize();
    let left = forward.cross(up.normalize());
    let true_up = left.cross(forward);

    let orientation = matrix4(
        left.x,     left.y,     left.z,     0.0,
        true_up.x,  true_up.y,  true_up.z,  0.0,
        -forward.x, -forward.y, -forward.z, 0.0,
        0.0,        0.0,        0.0,        1.0
    );
    return orientation * translation(-from.x, -from.y, -from.z);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_prelude::*;
    use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4};

    //macro_rules! assert_approx_eq {
    //    ($x:expr, $y:expr)  => (assert!(approx_eq!(f64, $x, $y, epsilon = 0.00001)))
    //}

    #[test]
    fn translation_multiply() {
        assert_eq!(translation(5.0, -3.0, 2.0) * point(-3.0, 4.0, 5.0),
                   point(2.0, 1.0, 7.0));
    }

    #[test]
    fn translation_inverse() {
        assert_eq!(translation(5.0, -3.0, 2.0).inverse() * point(-3.0, 4.0, 5.0),
                   point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_vector_no_effect() {
        let v = vector(-3.0, 4.0, 5.0);
        assert_eq!(translation(5.0, -3.0, 2.0) * v, v);
    }
    
    #[test]
    fn scaling_point() {
        assert_eq!(scaling(2.0, 3.0, 4.0) * point(-4.0, 6.0, 8.0),
                   point(-8.0, 18.0, 32.0));
    }
    
    #[test]
    fn scaling_vector() {
        assert_eq!(scaling(2.0, 3.0, 4.0) * vector(-4.0, 6.0, 8.0),
                   vector(-8.0, 18.0, 32.0));
    }
    
    #[test]
    fn scaling_inverse() {
        assert_eq!(scaling(2.0, 3.0, 4.0).inverse() * vector(-4.0, 6.0, 8.0),
                   vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn scaling_reflection() {
        assert_eq!(scaling(-1.0, 1.0, 1.0) * point(2.0, 3.0, 4.0),
                   point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotation_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        assert_eq!(rotation_x(FRAC_PI_4) * p, point(0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2));
        assert_eq!(rotation_x(FRAC_PI_2) * p, point(0.0, 0.0, 1.0));
    }

    #[test]
    fn rotation_x_inverse() {
        assert_eq!(rotation_x(FRAC_PI_4).inverse() * point(0.0, 1.0, 0.0),
                   point(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn rotation_y_axis() {
        let p = point(0.0, 0.0, 1.0);
        assert_eq!(rotation_y(FRAC_PI_4) * p, point(FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2));
        assert_eq!(rotation_y(FRAC_PI_2) * p, point(1.0, 0.0, 0.0));
    }
    
    #[test]
    fn rotation_z_axis() {
        let p = point(0.0, 1.0, 0.0);
        assert_eq!(rotation_z(FRAC_PI_4) * p, point(-FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0));
        assert_eq!(rotation_z(FRAC_PI_2) * p, point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_all() {
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * p, point(5.0, 3.0, 4.0));
        assert_eq!(shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0) * p, point(6.0, 3.0, 4.0));
        assert_eq!(shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0) * p, point(2.0, 5.0, 4.0));
        assert_eq!(shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0) * p, point(2.0, 7.0, 4.0));
        assert_eq!(shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0) * p, point(2.0, 3.0, 6.0));
        assert_eq!(shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0) * p, point(2.0, 3.0, 7.0));
    }

    #[test]
    fn transformation_sequence() {
        let p = point(1.0, 0.0, 1.0);
        let a = rotation_x(FRAC_PI_2);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        assert_eq!((c * b * a) * p, (c * (b * (a * p))));
        assert_eq!((c * b * a) * p, point(15.0, 0.0, 7.0));
    }

    #[test]
    fn view_transform_default_orientation() {
        assert_eq!(view_transform(point(0.0, 0.0, 0.0), point(0.0, 0.0, -1.0), vector(0.0, 1.0, 0.0)),
                   M4::IDENTITY);

    }

    #[test]
    fn view_transform_positive_z() {
        assert_eq!(view_transform(point(0.0, 0.0, 0.0), point(0.0, 0.0, 1.0), vector(0.0, 1.0, 0.0)),
                   scaling(-1.0, 1.0, -1.0));

    }

    #[test]
    fn view_transform_moves_the_world() {
        assert_eq!(view_transform(point(0.0, 0.0, 8.0), point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0)),
                   translation(0.0, 0.0, -8.0));

    }
    
    #[test]
    fn view_transform_arbitary_transform() {
        assert_eq!(view_transform(point(1.0, 3.0, 2.0), point(4.0, -2.0, 8.0), vector(1.0, 1.0, 0.0)),
                   parse_matrix4("
                       | -0.50709 | 0.50709 |  0.67612 | -2.36643 |
                       |  0.76772 | 0.60609 |  0.12122 | -2.82843 |
                       | -0.35857 | 0.59761 | -0.71714 |  0.00000 |
                       |  0.00000 | 0.00000 |  0.00000 |  1.00000 |
                   "));
    }
}
/*
TODO
Scenario: The transformation matrix for the default orientation
  Given from â† point(0, 0, 0)
    And to â† point(0, 0, -1)
    And up â† vector(0, 1, 0)
  When t â† view_transform(from, to, up)
  Then t = identity_matrix

Scenario: A view transformation matrix looking in positive z direction
  Given from â† point(0, 0, 0)
    And to â† point(0, 0, 1)
    And up â† vector(0, 1, 0)
  When t â† view_transform(from, to, up)
  Then t = scaling(-1, 1, -1)

Scenario: The view transformation moves the world
  Given from â† point(0, 0, 8)
    And to â† point(0, 0, 0)
    And up â† vector(0, 1, 0)
  When t â† view_transform(from, to, up)
  Then t = translation(0, 0, -8)

Scenario: An arbitrary view transformation
  Given from â† point(1, 3, 2)
    And to â† point(4, -2, 8)
    And up â† vector(1, 1, 0)
  When t â† view_transform(from, to, up)
  Then t is the following 4x4 matrix:
      | -0.50709 | 0.50709 |  0.67612 | -2.36643 |
      |  0.76772 | 0.60609 |  0.12122 | -2.82843 |
      | -0.35857 | 0.59761 | -0.71714 |  0.00000 |
      |  0.00000 | 0.00000 |  0.00000 |  1.00000 |
*/
