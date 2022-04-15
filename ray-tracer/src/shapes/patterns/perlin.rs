use crate::{
    primatives::{matrix::Matrix, point::Point, tuple::Tuple},
    P,
};

use super::{BoxedPattern, Pattern};

/// PerlinPattern applies a perlin noise jitter to the given pattern
#[derive(Debug)]
pub struct PerlinPattern {
    pattern: BoxedPattern,
    repeat: Option<usize>,
    transform: Matrix,
    inverse_transform: Matrix,
}

impl PerlinPattern {
    pub fn new(pattern: BoxedPattern, repeat: Option<usize>, transform: Option<Matrix>) -> Self {
        Self {
            pattern,
            repeat,
            transform: transform.clone().unwrap_or_default(),
            inverse_transform: transform.unwrap_or_default().inverse().unwrap(),
        }
    }
}

impl Pattern for PerlinPattern {
    fn local_color_at(&self, pattern_point: Point) -> crate::primatives::color::Color {
        let scale_value = 0.01;
        let octaves = 3;
        let persistance = 0.8;

        let jitter_x =
            pattern_point.x() + octave_perlin(pattern_point, octaves, persistance) * scale_value;
        let jitter_y = pattern_point.y()
            + octave_perlin(pattern_point + P![0., 0., 1.], octaves, persistance) * scale_value;
        let jitter_z = pattern_point.z()
            + octave_perlin(pattern_point + P![0., 0., 2.], octaves, persistance) * scale_value;

        let point = Point::new(jitter_x, jitter_y, jitter_z);

        self.pattern.local_color_at(point)
    }

    fn set_transformation(&mut self, transform: crate::primatives::matrix::Matrix) {
        self.transform = transform.clone();
        self.inverse_transform = transform
            .inverse()
            .expect("trying to invert a matrix that cannot be inverted");
    }

    fn inverse_transformation(&self) -> &crate::primatives::matrix::Matrix {
        &self.inverse_transform
    }

    fn box_clone(&self) -> super::BoxedPattern {
        Box::new(self.clone())
    }

    fn box_eq(&self, other: &dyn std::any::Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Clone for PerlinPattern {
    fn clone(&self) -> Self {
        Self {
            pattern: self.pattern.box_clone(),
            repeat: self.repeat,
            transform: self.transform.clone(),
            inverse_transform: self.inverse_transform.clone(),
        }
    }
}

impl PartialEq for PerlinPattern {
    fn eq(&self, other: &Self) -> bool {
        self.pattern.box_eq(&other.pattern)
            && self.repeat == other.repeat
            && self.transform == other.transform
            && self.inverse_transform == other.inverse_transform
    }
}

// Hash lookup table as defined by Ken Perlin.  This is a randomly
// arranged array of all numbers from 0-255 inclusive. repeated twice
const PERMUTATION: [usize; 512] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180, 151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194,
    233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234,
    75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174,
    20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83,
    111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25,
    63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188,
    159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147,
    118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
    213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253,
    19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193,
    238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31,
    181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93,
    222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
];

pub fn perlin_noise(point: Point, repeat: Option<usize>) -> f64 {
    let x_i: usize = (point.x() as usize) & 255;
    let y_i: usize = (point.y() as usize) & 255;
    let z_i: usize = (point.z() as usize) & 255;
    let x_f = point.x().fract();
    let y_f = point.y().fract();
    let z_f = point.z().fract();

    let u = fade(x_f);
    let v = fade(y_f);
    let w = fade(z_f);

    let p = PERMUTATION;
    let aaa = p[p[p[x_i] + y_i] + z_i];
    let aba = p[p[p[x_i] + inc(y_i, repeat)] + z_i];
    let aab = p[p[p[x_i] + y_i] + inc(z_i, repeat)];
    let abb = p[p[p[x_i] + inc(y_i, repeat)] + inc(z_i, repeat)];
    let baa = p[p[p[inc(x_i, repeat)] + y_i] + z_i];
    let bba = p[p[p[inc(x_i, repeat)] + inc(y_i, repeat)] + z_i];
    let bab = p[p[p[inc(x_i, repeat)] + y_i] + inc(z_i, repeat)];
    let bbb = p[p[p[inc(x_i, repeat)] + inc(y_i, repeat)] + inc(z_i, repeat)];

    // The gradient function calculates the dot product between a pseudorandom
    // gradient vector and the vector from the input coordinate to the 8
    // surrounding points in its unit cube.
    let mut x_1 = lerp(grad(aaa, x_f, y_f, z_f), grad(baa, x_f - 1.0, y_f, z_f), u);

    // This is all then lerped together as a sort of weighted average based on the faded (u,v,w)
    // values we made earlier.
    let mut x_2 = lerp(
        grad(aba, x_f, y_f - 1.0, z_f),
        grad(bba, x_f - 1.0, y_f - 1.0, z_f),
        u,
    );
    let y_1 = lerp(x_1, x_2, v);

    x_1 = lerp(
        grad(aab, x_f, y_f, z_f - 1.0),
        grad(bab, x_f - 1.0, y_f, z_f - 1.0),
        u,
    );
    x_2 = lerp(
        grad(abb, x_f, y_f - 1.0, z_f - 1.0),
        grad(bbb, x_f - 1.0, y_f - 1.0, z_f - 1.0),
        u,
    );
    let y_2 = lerp(x_1, x_2, v);

    (lerp(y_1, y_2, w) + 1.0) / 2.0
}

// Fade function as defined by Ken Perlin.  This eases coordinate values
// so that they will ease towards integral values.  This ends up smoothing
// the final output.
// 6t^5 - 15t^4 + 10t^3
fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn inc(mut num: usize, repeat: Option<usize>) -> usize {
    num += 1;
    if let Some(repeat) = repeat {
        num %= repeat
    }
    num
}

// Source: http://riven8192.blogspot.com/2010/08/calculate-perlinnoise-twice-as-fast.html
fn grad(hash: usize, x: f64, y: f64, z: f64) -> f64 {
    match hash & 0xF {
        0x0 => x + y,
        0x1 => -x + y,
        0x2 => x - y,
        0x3 => -x - y,
        0x4 => x + z,
        0x5 => -x + z,
        0x6 => x - z,
        0x7 => -x - z,
        0x8 => y + z,
        0x9 => -y + z,
        0xA => y - z,
        0xB => -y - z,
        0xC => y + x,
        0xD => -y + z,
        0xE => y - x,
        0xF => -y - z,
        _ => unreachable!(), // cant happen
    }
}

// Linear Interpolate
fn lerp(a: f64, b: f64, x: f64) -> f64 {
    a + x * (b - a)
}

/// octave_perlin applies a series of "octaves" of perlin noise to a point.
/// each perlin ocatve has a lower effect on the overall shape.
fn octave_perlin(point: Point, octaves: usize, persistance: f64) -> f64 {
    let mut total = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    let mut max_value = 0.0; // Used for normalizing result to 0.0 - 1.0
    for _ in [0..octaves] {
        total += perlin_noise(point * frequency, None) * amplitude;

        max_value += amplitude;

        amplitude *= persistance;
        frequency *= 2.0;
    }

    total / max_value
}
