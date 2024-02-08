use std::sync::OnceLock;

use crate::utils::{
    color_utils::{white_point_d65, y_from_lstar},
    math_utils::lerp,
};

/// In traditional color spaces, a color can be identified solely by the
/// observer's measurement of the color. Color appearance models such as CAM16
/// also use information about the environment where the color was
/// observed, known as the viewing conditions.
///
/// For example, white under the traditional assumption of a midday sun white
/// point is accurately measured as a slightly chromatic blue by CAM16. (roughly,
/// hue 203, chroma 3, lightness 100)
///
/// This struct caches intermediate values of the CAM16 conversion process that
/// depend only on viewing conditions, enabling speed ups.
pub struct ViewingConditions {
    /// Parameters are intermediate values of the CAM16 conversion process. Their
    /// names are shorthand for technical color science terminology, this struct
    /// would not benefit from documenting them individually. A brief overview
    /// is available in the CAM16 specification, and a complete overview requires
    /// a color science textbook, such as Fairchild's Color Appearance Models.
    pub n: f64,
    pub aw: f64,
    pub nbb: f64,
    pub ncb: f64,
    pub c: f64,
    pub nc: f64,
    pub rgb_d: [f64; 3],
    pub fl: f64,
    pub f_l_root: f64,
    pub z: f64,
}

impl ViewingConditions {
    /// Create ViewingConditions from a simple, physically relevant, set of
    /// parameters.
    ///
    /// # Arguments
    ///
    /// * `white_point` - White point, measured in the XYZ color space.
    ///     default = D65, or sunny day afternoon
    /// * `adapting_luminance` - The luminance of the adapting field. Informally,
    ///     how bright it is in the room where the color is viewed. Can be
    ///     calculated from lux by multiplying lux by 0.0586. default = 11.72,
    ///     or 200 lux.
    /// * `background_lstar` - The lightness of the area surrounding the color.
    ///     measured by L* in L*a*b*. default = 50.0
    /// * `surround` - A general description of the lighting surrounding the
    ///     color. 0 is pitch dark, like watching a movie in a theater. 1.0 is a
    ///     dimly light room, like watching TV at home at night. 2.0 means there
    ///     is no difference between the lighting on the color and around it.
    ///     default = 2.0
    /// * `discounting_illuminant` - Whether the eye accounts for the tint of the
    ///     ambient lighting, such as knowing an apple is still red in green light.
    ///     default = false, the eye does not perform this process on
    ///       self-luminous objects like displays.
    pub fn new(
        white_point: [f64; 3],
        adapting_luminance: f64,
        background_lstar: f64,
        surround: f64,
        discounting_illuminant: bool,
    ) -> ViewingConditions {
        let xyz = white_point;
        let r_w = xyz[0] * 0.401288 + xyz[1] * 0.650173 + xyz[2] * -0.051461;
        let g_w = xyz[0] * -0.250268 + xyz[1] * 1.204414 + xyz[2] * 0.045854;
        let b_w = xyz[0] * -0.002079 + xyz[1] * 0.048952 + xyz[2] * 0.953127;
        let f = 0.8 + surround / 10.0;
        let c = if f >= 0.9 {
            lerp(0.59, 0.69, (f - 0.9) * 10.0)
        } else {
            lerp(0.525, 0.59, (f - 0.8) * 10.0)
        };
        let mut d = if discounting_illuminant {
            1.0
        } else {
            f * (1.0 - (1.0 / 3.6) * ((-adapting_luminance - 42.0) / 92.0).exp())
        };
        d = if d > 1.0 {
            1.0
        } else if d < 0.0 {
            0.0
        } else {
            d
        };
        let nc = f;
        let rgb_d = [
            d * (100.0 / r_w) + 1.0 - d,
            d * (100.0 / g_w) + 1.0 - d,
            d * (100.0 / b_w) + 1.0 - d,
        ];
        let k = 1.0 / (5.0 * adapting_luminance + 1.0);
        let k4 = k * k * k * k;
        let k4_f = 1.0 - k4;
        let fl = k4 * adapting_luminance + 0.1 * k4_f * k4_f * (5.0 * adapting_luminance).cbrt();
        let n = y_from_lstar(background_lstar) / white_point[1];
        let z = 1.48 + n.sqrt();
        let nbb = 0.725 / n.powf(0.2);
        let ncb = nbb;
        let rgb_a_factors = [
            ((fl * rgb_d[0] * r_w) / 100.0).powf(0.42),
            ((fl * rgb_d[1] * g_w) / 100.0).powf(0.42),
            ((fl * rgb_d[2] * b_w) / 100.0).powf(0.42),
        ];
        let rgb_a = [
            (400.0 * rgb_a_factors[0]) / (rgb_a_factors[0] + 27.13),
            (400.0 * rgb_a_factors[1]) / (rgb_a_factors[1] + 27.13),
            (400.0 * rgb_a_factors[2]) / (rgb_a_factors[2] + 27.13),
        ];
        let aw = (2.0 * rgb_a[0] + rgb_a[1] + 0.05 * rgb_a[2]) * nbb;
        ViewingConditions {
            n,
            aw,
            nbb,
            ncb,
            c,
            nc,
            rgb_d,
            fl,
            f_l_root: fl.powf(0.25),
            z,
        }
    }
}

pub fn default() -> &'static ViewingConditions {
    static DEFAULT: OnceLock<ViewingConditions> = OnceLock::new();
    DEFAULT.get_or_init(|| ViewingConditions::default())
}

impl Default for ViewingConditions {
    fn default() -> Self {
        ViewingConditions::new(
            white_point_d65(),
            (200.0 / std::f64::consts::PI) * y_from_lstar(50.0) / 100.0,
            50.0,
            2.0,
            false,
        )
    }
}
