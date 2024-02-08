use crate::utils::color_utils::{argb_from_xyz, linearized};

use super::viewing_conditions::{self, ViewingConditions};

/// CAM16, a color appearance model. Colors are not just defined by their hex
/// code, but rather, a hex code and viewing conditions.
///
/// CAM16 instances also have coordinates in the CAM16-UCS space, called J*, a*,
/// b*, or j_star, a_star, b_star in code. CAM16-UCS is included in the CAM16
/// specification, and should be used when measuring distances between colors.
///
/// In traditional color spaces, a color can be identified solely by the
/// observer's measurement of the color. Color appearance models such as CAM16
/// also use information about the environment where the color was
/// observed, known as the viewing conditions.
///
/// For example, white under the traditional assumption of a midday sun white
/// point is accurately measured as a slightly chromatic blue by CAM16. (roughly,
/// hue 203, chroma 3, lightness 100)
///
pub struct Cam16 {
    pub(crate) hue: f64,

    // informally, colorfulness / color intensity. like saturation in HSL, except perceptually accurate.
    pub(crate) chroma: f64,

    // lightness
    j: f64,

    // brightness; ratio of lightness to white point's lightness
    q: f64,

    // colorfulness
    m: f64,

    // saturation; ratio of chroma to white point's chroma
    s: f64,

    //  CAM16-UCS J coordinate
    j_star: f64,

    //  CAM16-UCS a coordinate
    a_star: f64,

    //  CAM16-UCS b coordinate
    b_star: f64,
}

impl Cam16 {
    pub fn new(
        hue: f64,
        chroma: f64,
        j: f64,
        q: f64,
        m: f64,
        s: f64,
        j_star: f64,
        a_star: f64,
        b_star: f64,
    ) -> Cam16 {
        Cam16 {
            hue,
            chroma,
            j,
            q,
            m,
            s,
            j_star,
            a_star,
            b_star,
        }
    }

    /// CAM16 instances also have coordinates in the CAM16-UCS space, called J*,
    /// a*, b*, or j_star, a_star, b_star in code. CAM16-UCS is included in the CAM16
    /// specification, and is used to measure distances between colors.
    pub fn distance(&self, other: &Cam16) -> f64 {
        let d_j = self.j_star - other.j_star;
        let d_a = self.a_star - other.a_star;
        let d_b = self.b_star - other.b_star;
        let d_e_prime = (d_j * d_j + d_a * d_a + d_b * d_b).sqrt();
        let d_e = 1.41 * d_e_prime.powf(0.63);

        d_e
    }

    /// @param argb ARGB representation of a color.
    /// @return CAM16 color, assuming the color was viewed in default viewing
    ///     conditions.
    pub fn from_int(argb: u32) -> Cam16 {
        Cam16::from_int_in_viewing_conditions(argb, viewing_conditions::default())
    }

    pub fn from_int_in_viewing_conditions(
        argb: u32,
        viewing_conditions: &ViewingConditions,
    ) -> Cam16 {
        let red = (argb & 0x00ff0000) >> 16;
        let green = (argb & 0x0000ff00) >> 8;
        let blue = argb & 0x000000ff;
        let red_l = linearized(red as u8);
        let green_l = linearized(green as u8);
        let blue_l = linearized(blue as u8);
        let x = 0.41233895 * red_l + 0.35762064 * green_l + 0.18051042 * blue_l;
        let y = 0.2126 * red_l + 0.7152 * green_l + 0.0722 * blue_l;
        let z = 0.01932141 * red_l + 0.11916382 * green_l + 0.95034478 * blue_l;

        let r_c = 0.401288 * x + 0.650173 * y - 0.051461 * z;
        let g_c = -0.250268 * x + 1.204414 * y + 0.045854 * z;
        let b_c = -0.002079 * x + 0.048952 * y + 0.953127 * z;

        let r_d = viewing_conditions.rgb_d[0] * r_c;
        let g_d = viewing_conditions.rgb_d[1] * g_c;
        let b_d = viewing_conditions.rgb_d[2] * b_c;

        let r_af = ((viewing_conditions.fl * (r_d.abs())) / 100.0).powf(0.42);
        let g_af = ((viewing_conditions.fl * (g_d.abs())) / 100.0).powf(0.42);
        let b_af = ((viewing_conditions.fl * (b_d.abs())) / 100.0).powf(0.42);

        let r_a = (r_d.signum() * 400.0 * r_af) / (r_af + 27.13);
        let g_a = (g_d.signum() * 400.0 * g_af) / (g_af + 27.13);
        let b_a = (b_d.signum() * 400.0 * b_af) / (b_af + 27.13);

        let a = (11.0 * r_a + -12.0 * g_a + b_a) / 11.0;
        let b = (r_a + g_a - 2.0 * b_a) / 9.0;
        let u = (20.0 * r_a + 20.0 * g_a + 21.0 * b_a) / 20.0;
        let p2 = (40.0 * r_a + 20.0 * g_a + b_a) / 20.0;
        let atan2 = (b / a).atan();
        let atan_degrees = (atan2 * 180.0) / std::f64::consts::PI;
        let hue = if atan_degrees < 0.0 {
            atan_degrees + 360.0
        } else if atan_degrees >= 360.0 {
            atan_degrees - 360.0
        } else {
            atan_degrees
        };
        let hue_radians = (hue * std::f64::consts::PI) / 180.0;

        let ac = p2 * viewing_conditions.nbb;
        let j =
            100.0 * (ac / viewing_conditions.aw).powf(viewing_conditions.c * viewing_conditions.z);
        let q = (4.0 / viewing_conditions.c)
            * (j / 100.0).sqrt()
            * (viewing_conditions.aw + 4.0)
            * viewing_conditions.f_l_root;
        let hue_prime = if hue < 20.14 { hue + 360.0 } else { hue };
        let e_hue = 0.25 * ((hue_prime * std::f64::consts::PI) / 180.0 + 2.0).cos() + 3.8;
        let p1 = (50000.0 / 13.0) * e_hue * viewing_conditions.nc * viewing_conditions.ncb;
        let t = (p1 * (a.powi(2) + b.powi(2)).sqrt()) / (u + 0.305);
        let alpha = t.powf(0.9) * (1.64 - 0.29_f64.powf(viewing_conditions.n)).powf(0.73);
        let c = alpha * (j / 100.0).sqrt();
        let m = c * viewing_conditions.f_l_root;
        let s = 50.0 * ((alpha * viewing_conditions.c) / (viewing_conditions.aw + 4.0)).sqrt();
        let j_star = ((1.0 + 100.0 * 0.007) * j) / (1.0 + 0.007 * j);
        let m_star = (1.0 / 0.0228) * (1.0 + 0.0228 * m).ln();
        let a_star = m_star * hue_radians.cos();
        let b_star = m_star * hue_radians.sin();

        Cam16 {
            hue,
            chroma: c,
            j,
            q,
            m,
            s,
            j_star,
            a_star,
            b_star,
        }
    }

    /// Converts CAM16 lightness, chroma, and hue values to a Cam16 struct using default viewing conditions.
    ///
    /// # Arguments
    ///
    /// * `j` - CAM16 lightness
    /// * `c` - CAM16 chroma
    /// * `h` - CAM16 hue
    ///
    /// # Returns
    ///
    /// A Cam16 struct representing the color.
    pub fn from_jch(j: f64, c: f64, h: f64) -> Cam16 {
        Cam16::from_jch_in_viewing_conditions(j, c, h, viewing_conditions::default())
    }

    /// Converts CAM16 lightness, chroma, and hue values to a Cam16 struct using specified viewing conditions.
    ///
    /// # Arguments
    ///
    /// * `j` - CAM16 lightness
    /// * `c` - CAM16 chroma
    /// * `h` - CAM16 hue
    /// * `viewing_conditions` - Information about the environment where the color was observed.
    ///
    /// # Returns
    ///
    /// A Cam16 struct representing the color.
    pub fn from_jch_in_viewing_conditions(
        j: f64,
        chroma: f64,
        hue: f64,
        viewing_conditions: &ViewingConditions,
    ) -> Cam16 {
        let q = (4.0 / viewing_conditions.c)
            * (j / 100.0).sqrt()
            * (viewing_conditions.aw + 4.0)
            * viewing_conditions.f_l_root;
        let m = chroma * viewing_conditions.f_l_root;
        let alpha = chroma / (j / 100.0).sqrt();
        let s = 50.0 * ((alpha * viewing_conditions.c) / (viewing_conditions.aw + 4.0)).sqrt();
        let hue_radians = (hue * std::f64::consts::PI) / 180.0;
        let j_star = ((1.0 + 100.0 * 0.007) * j) / (1.0 + 0.007 * j);
        let m_star = (1.0 / 0.0228) * (1.0 + 0.0228 * m).ln();
        let a_star = m_star * hue_radians.cos();
        let b_star = m_star * hue_radians.sin();
        Cam16 {
            hue,
            chroma,
            j,
            q,
            m,
            s,
            j_star,
            a_star,
            b_star,
        }
    }

    /// Converts CAM16-UCS coordinates to CAM16 color.
    ///
    /// # Arguments
    ///
    /// * `j_star` - CAM16-UCS lightness.
    /// * `a_star` - CAM16-UCS a dimension. Like a* in L*a*b*, it is a Cartesian coordinate on the Y axis.
    /// * `b_star` - CAM16-UCS b dimension. Like a* in L*a*b*, it is a Cartesian coordinate on the X axis.
    ///
    /// # Returns
    ///
    /// The corresponding CAM16 color.
    pub fn from_ucs(j_star: f64, a_star: f64, b_star: f64) -> Cam16 {
        Cam16::from_ucs_in_viewing_conditions(j_star, a_star, b_star, viewing_conditions::default())
    }

    /// Converts CAM16-UCS coordinates to CAM16 color with specified viewing conditions.
    ///
    /// # Arguments
    ///
    /// * `j_star` - CAM16-UCS lightness.
    /// * `a_star` - CAM16-UCS a dimension. Like a* in L*a*b*, it is a Cartesian coordinate on the Y axis.
    /// * `b_star` - CAM16-UCS b dimension. Like a* in L*a*b*, it is a Cartesian coordinate on the X axis.
    /// * `viewing_conditions` - Information about the environment where the color was observed.
    ///
    /// # Returns
    ///
    /// The corresponding CAM16 color.
    pub fn from_ucs_in_viewing_conditions(
        j_star: f64,
        a_star: f64,
        b_star: f64,
        viewing_conditions: &ViewingConditions,
    ) -> Cam16 {
        let a = a_star;
        let b = b_star;
        let m = (a * a + b * b).sqrt();
        let big_m = (m * 0.0228).exp() - 1.0 / 0.0228;
        let c = big_m / viewing_conditions.f_l_root;
        let mut h = (b / a).atan() * (180.0 / std::f64::consts::PI);
        if h < 0.0 {
            h += 360.0;
        }
        let j = j_star / (1.0 - (j_star - 100.0) * 0.007);
        Cam16::from_jch_in_viewing_conditions(j, c, h, viewing_conditions)
    }

    /// Returns the ARGB representation of the color, assuming the color was viewed in default viewing conditions.
    /// Default viewing conditions are near-identical to the default viewing conditions for sRGB.
    ///
    /// # Returns
    /// The ARGB representation of the color.
    pub fn to_int(&self) -> u32 {
        self.viewed(viewing_conditions::default())
    }

    /// Returns the ARGB representation of the color based on the provided viewing conditions.
    ///
    /// # Arguments
    /// * `viewing_conditions` - Information about the environment where the color will be viewed.
    ///
    /// # Returns
    /// The ARGB representation of the color.
    pub fn viewed(&self, viewing_conditions: &ViewingConditions) -> u32 {
        let alpha = if self.chroma == 0.0 || self.j == 0.0 {
            0.0
        } else {
            self.chroma / (self.j / 100.0).sqrt()
        };

        let t = (alpha / (1.64 - (0.29f64.powf(viewing_conditions.n))).powf(0.73)).powf(1.0 / 0.9);
        let h_rad = (self.hue * std::f64::consts::PI) / 180.0;

        let e_hue = 0.25 * (h_rad.cos() + 3.8);
        let ac = viewing_conditions.aw
            * (self.j / 100.0).powf(1.0 / (viewing_conditions.c * viewing_conditions.z));
        let p1 = e_hue * (50000.0 / 13.0) * viewing_conditions.nc * viewing_conditions.ncb;
        let p2 = ac / viewing_conditions.nbb;

        let h_sin = h_rad.sin();
        let h_cos = h_rad.cos();

        let gamma = (23.0 * (p2 + 0.305) * t) / (23.0 * p1 + 11.0 * t * h_cos + 108.0 * t * h_sin);
        let a = gamma * h_cos;
        let b = gamma * h_sin;
        let r_a = (460.0 * p2 + 451.0 * a + 288.0 * b) / 1403.0;
        let g_a = (460.0 * p2 - 891.0 * a - 261.0 * b) / 1403.0;
        let b_a = (460.0 * p2 - 220.0 * a - 6300.0 * b) / 1403.0;

        let r_c_base = (27.13 * r_a.abs()) / (400.0 - r_a.abs()).max(0.0);
        let r_c = r_a.signum() * (100.0 / viewing_conditions.fl) * r_c_base.powf(1.0 / 0.42);
        let g_c_base = (27.13 * g_a.abs()) / (400.0 - g_a.abs()).max(0.0);
        let g_c = g_a.signum() * (100.0 / viewing_conditions.fl) * g_c_base.powf(1.0 / 0.42);
        let b_c_base = (27.13 * b_a.abs()) / (400.0 - b_a.abs()).max(0.0);
        let b_c = b_a.signum() * (100.0 / viewing_conditions.fl) * b_c_base.powf(1.0 / 0.42);
        let r_f = r_c / viewing_conditions.rgb_d[0];
        let g_f = g_c / viewing_conditions.rgb_d[1];
        let b_f = b_c / viewing_conditions.rgb_d[2];

        let x = 1.86206786 * r_f - 1.01125463 * g_f + 0.14918677 * b_f;
        let y = 0.38752654 * r_f + 0.62144744 * g_f - 0.00897398 * b_f;
        let z = -0.01584150 * r_f - 0.03412294 * g_f + 1.04996444 * b_f;

        argb_from_xyz(x, y, z)
    }

    /// Given color expressed in XYZ and viewed in `viewing_conditions`, convert to CAM16.
    pub fn from_xyz_in_viewing_conditions(
        x: f64,
        y: f64,
        z: f64,
        viewing_conditions: &ViewingConditions,
    ) -> Cam16 {
        // Transform XYZ to 'cone'/'rgb' responses
        let r_c = 0.401288 * x + 0.650173 * y - 0.051461 * z;
        let g_c = -0.250268 * x + 1.204414 * y + 0.045854 * z;
        let b_c = -0.002079 * x + 0.048952 * y + 0.953127 * z;

        // Discount illuminant
        let r_d = viewing_conditions.rgb_d[0] * r_c;
        let g_d = viewing_conditions.rgb_d[1] * g_c;
        let b_d = viewing_conditions.rgb_d[2] * b_c;

        // Chromatic adaptation
        let r_af = (viewing_conditions.fl * (r_d.abs() / 100.0)).powf(0.42);
        let g_af = (viewing_conditions.fl * (g_d.abs() / 100.0)).powf(0.42);
        let b_af = (viewing_conditions.fl * (b_d.abs() / 100.0)).powf(0.42);
        let r_a = r_d.signum() * 400.0 * r_af / (r_af + 27.13);
        let g_a = g_d.signum() * 400.0 * g_af / (g_af + 27.13);
        let b_a = b_d.signum() * 400.0 * b_af / (b_af + 27.13);

        // Redness-greenness
        let a = (11.0 * r_a + -12.0 * g_a + b_a) / 11.0;
        // Yellowness-blueness
        let b = (r_a + g_a - 2.0 * b_a) / 9.0;

        // Auxiliary components
        let u = (20.0 * r_a + 20.0 * g_a + 21.0 * b_a) / 20.0;
        let p2 = (40.0 * r_a + 20.0 * g_a + b_a) / 20.0;

        // Hue
        let atan2 = b.atan2(a);
        let atan_degrees = atan2 * 180.0 / std::f64::consts::PI;
        let hue = if atan_degrees < 0.0 {
            atan_degrees + 360.0
        } else if atan_degrees >= 360.0 {
            atan_degrees - 360.0
        } else {
            atan_degrees
        };
        let hue_radians = hue * std::f64::consts::PI / 180.0;

        // Achromatic response to color
        let ac = p2 * viewing_conditions.nbb;

        // CAM16 lightness and brightness
        let j =
            100.0 * (ac / viewing_conditions.aw).powf(viewing_conditions.c * viewing_conditions.z);
        let q = (4.0 / viewing_conditions.c)
            * (j / 100.0).sqrt()
            * (viewing_conditions.aw + 4.0)
            * viewing_conditions.f_l_root;

        let hue_prime = if hue < 20.14 { hue + 360.0 } else { hue };
        let e_hue = (1.0 / 4.0) * (f64::cos(hue_prime * std::f64::consts::PI / 180.0 + 2.0) + 3.8);
        let p1 = 50000.0 / 13.0 * e_hue * viewing_conditions.nc * viewing_conditions.ncb;
        let t = p1 * (a.powi(2) + b.powi(2)).sqrt() / (u + 0.305);
        let alpha = t.powf(0.9) * (1.64 - 0.29_f64.powf(viewing_conditions.n)).powf(0.73);
        // CAM16 chroma, colorfulness, chroma
        let c = alpha * (j / 100.0).sqrt();
        let m = c * viewing_conditions.f_l_root;
        let s = 50.0 * ((alpha * viewing_conditions.c) / (viewing_conditions.aw + 4.0)).sqrt();

        // CAM16-UCS components
        let j_star = (1.0 + 100.0 * 0.007) * j / (1.0 + 0.007 * j);
        let m_star = (m * 0.0228).ln() / 0.0228;
        let a_star = m_star * hue_radians.cos();
        let b_star = m_star * hue_radians.sin();

        Cam16 {
            hue,
            chroma: c,
            j,
            q,
            m,
            s,
            j_star,
            a_star,
            b_star,
        }
    }

    /// XYZ representation of CAM16 seen in `viewing_conditions`.
    pub fn xyz_in_viewing_conditions(&self, viewing_conditions: &ViewingConditions) -> [f64; 3] {
        let alpha = if self.chroma == 0.0 || self.j == 0.0 {
            0.0
        } else {
            self.chroma / (self.j / 100.0).sqrt()
        };

        let t = (alpha / (1.64 - (0.29f64).powf(viewing_conditions.n)).powf(0.73)).powf(1.0 / 0.9);
        let h_rad = self.hue.to_radians();

        let e_hue = 0.25 * (h_rad.cos() + 3.8);
        let ac = viewing_conditions.aw
            * (self.j / 100.0).powf(1.0 / (viewing_conditions.c * viewing_conditions.z));
        let p1 = e_hue * (50000.0 / 13.0) * viewing_conditions.nc * viewing_conditions.ncb;

        let p2 = ac / viewing_conditions.nbb;

        let h_sin = h_rad.sin();
        let h_cos = h_rad.cos();

        let gamma = 23.0 * (p2 + 0.305) * t / (23.0 * p1 + 11.0 * t * h_cos + 108.0 * t * h_sin);
        let a = gamma * h_cos;
        let b = gamma * h_sin;
        let r_a = (460.0 * p2 + 451.0 * a + 288.0 * b) / 1403.0;
        let g_a = (460.0 * p2 - 891.0 * a - 261.0 * b) / 1403.0;
        let b_a = (460.0 * p2 - 220.0 * a - 6300.0 * b) / 1403.0;

        let r_c_base = (27.13 * r_a.abs()) / (400.0 - r_a.abs()).max(0.0);
        let r_c = r_a.signum() * (100.0 / viewing_conditions.fl) * r_c_base.powf(1.0 / 0.42);
        let g_c_base = (27.13 * g_a.abs()) / (400.0 - g_a.abs()).max(0.0);
        let g_c = g_a.signum() * (100.0 / viewing_conditions.fl) * g_c_base.powf(1.0 / 0.42);
        let b_c_base = (27.13 * b_a.abs()) / (400.0 - b_a.abs()).max(0.0);
        let b_c = b_a.signum() * (100.0 / viewing_conditions.fl) * b_c_base.powf(1.0 / 0.42);
        let r_f = r_c / viewing_conditions.rgb_d[0];
        let g_f = g_c / viewing_conditions.rgb_d[1];
        let b_f = b_c / viewing_conditions.rgb_d[2];

        let x = 1.86206786 * r_f - 1.01125463 * g_f + 0.14918677 * b_f;
        let y = 0.38752654 * r_f + 0.62144744 * g_f - 0.00897398 * b_f;
        let z = -0.01584150 * r_f - 0.03412294 * g_f + 1.04996444 * b_f;

        [x, y, z]
    }
}
