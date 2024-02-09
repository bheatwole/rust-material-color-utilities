use crate::utils::color_utils::{lstar_from_argb, lstar_from_y};

use super::{cam16::Cam16, solve_to_int, viewing_conditions::ViewingConditions};

/// A color system built using CAM16 hue and chroma, and L* from
/// L*a*b*.
///
/// Using L* creates a link between the color system, contrast, and thus
/// accessibility. Contrast ratio depends on relative luminance, or Y in the XYZ
/// color space. L*, or perceptual luminance can be calculated from Y.
///
/// Unlike Y, L* is linear to human perception, allowing trivial creation of
/// accurate color tones.
///
/// Unlike contrast ratio, measuring contrast in L* is linear, and simple to
/// calculate. A difference of 40 in HCT tone guarantees a contrast ratio >= 3.0,
/// and a difference of 50 guarantees a contrast ratio >= 4.5.
///
///  HCT, hue, chroma, and tone. A color system that provides a perceptually
/// accurate color measurement system that can also accurately render what colors
/// will appear as in different lighting environments.

pub struct Hct {
    /// hue 0 <= hue < 360; invalid values are corrected.
    hue: f64,

    /// chroma 0 <= chroma < ?; Informally, colorfulness. The color
    ///     returned may be lower than the requested chroma. Chroma has a different
    ///     maximum for any given hue and tone.
    chroma: f64,

    /// tone 0 <= tone <= 100; invalid values are corrected.
    tone: f64,

    /// The ARGB color code associated with this Hct
    argb: u32,
}

impl Hct {
    pub fn from_hct(hue: f64, chroma: f64, tone: f64) -> Hct {
        Hct::from_int(solve_to_int(hue, chroma, tone))
    }

    pub fn from_int(argb: u32) -> Hct {
        let mut hct = Hct {
            hue: 0_f64,
            chroma: 0_f64,
            tone: 0_f64,
            argb,
        };
        hct.set_hct_from_int(argb);

        hct
    }

    fn set_hct_from_int(&mut self, argb: u32) {
        let cam = Cam16::from_int(argb);

        self.hue = cam.hue;
        self.chroma = cam.chroma;
        self.tone = lstar_from_argb(argb);
        self.argb = argb;
    }

    pub fn to_int(&self) -> u32 {
        self.argb
    }

    pub fn hue(&self) -> f64 {
        self.hue
    }

    pub fn set_hue(&mut self, hue: f64) {
        self.set_hct_from_int(solve_to_int(hue, self.chroma, self.tone))
    }

    pub fn chroma(&self) -> f64 {
        self.chroma
    }

    pub fn set_chroma(&mut self, chroma: f64) {
        self.set_hct_from_int(solve_to_int(self.hue, chroma, self.tone))
    }

    pub fn tone(&self) -> f64 {
        self.tone
    }

    pub fn set_tone(&mut self, tone: f64) {
        self.set_hct_from_int(solve_to_int(self.hue, self.chroma, tone))
    }

    /// Translates a color into different [ViewingConditions].
    ///
    /// Colors change appearance. They look different with lights on versus off,
    /// the same color, as in hex code, on white looks different when on black.
    /// This is called color relativity, most famously explicated by Josef Albers
    /// in Interaction of Color.
    ///
    /// In color science, color appearance models can account for this and
    /// calculate the appearance of a color in different settings. HCT is based on
    /// CAM16, a color appearance model, and uses it to make these calculations.
    ///
    /// See [ViewingConditions.new] for parameters affecting color appearance.
    pub fn in_viewing_conditions(&self, viewing_conditions: &ViewingConditions) -> Hct {
        // 1. Use CAM16 to find XYZ coordinates of color in specified VC.
        let cam = Cam16::from_int(self.to_int());
        let viewed_in_viewing_conditions = cam.xyz_in_viewing_conditions(viewing_conditions);

        // 2. Create CAM16 of those XYZ coordinates in default VC.
        let recast_in_vc = Cam16::from_xyz_in_viewing_conditions(
            viewed_in_viewing_conditions[0],
            viewed_in_viewing_conditions[1],
            viewed_in_viewing_conditions[2],
            super::viewing_conditions::default(),
        );

        // 3. Create HCT from:
        // - CAM16 using default VC with XYZ coordinates in specified VC.
        // - L* converted from Y in XYZ coordinates in specified VC.
        Hct::from_hct(
            recast_in_vc.hue,
            recast_in_vc.chroma,
            lstar_from_y(viewed_in_viewing_conditions[1]),
        )
    }
}
