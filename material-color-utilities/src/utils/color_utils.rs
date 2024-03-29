use super::math_utils::*;

/// Color science utilities.
///
/// Utility methods for color science constants and color space
/// conversions that aren't HCT or CAM16.
const SRGB_TO_XYZ: [[f64; 3]; 3] = [
    [0.41233895, 0.35762064, 0.18051042],
    [0.2126, 0.7152, 0.0722],
    [0.01932141, 0.11916382, 0.95034478],
];

const XYZ_TO_SRGB: [[f64; 3]; 3] = [
    [
        3.2413774792388685,
        -1.5376652402851851,
        -0.49885366846268053,
    ],
    [-0.9691452513005321, 1.8758853451067872, 0.04156585616912061],
    [
        0.05562093689691305,
        -0.20395524564742123,
        1.0571799111220335,
    ],
];

const WHITE_POINT_D65: [f64; 3] = [95.047, 100.0, 108.883];

/// Converts a color from RGB components to ARGB format.
pub fn argb_from_rgb(red: u8, green: u8, blue: u8) -> u32 {
    255 << 24 | (red as u32 & 255) << 16 | (green as u32 & 255) << 8 | blue as u32 & 255
}

/// Converts a color from linear RGB components to ARGB format.
pub fn argb_from_linrgb(linrgb: &[f64; 3]) -> u32 {
    let r = delinearized(linrgb[0]);
    let g = delinearized(linrgb[1]);
    let b = delinearized(linrgb[2]);
    argb_from_rgb(r, g, b)
}

/// Returns the alpha component of a color in ARGB format.
pub fn alpha_from_argb(argb: u32) -> u8 {
    (argb >> 24 & 255) as u8
}

/// Returns the red component of a color in ARGB format.
pub fn red_from_argb(argb: u32) -> u8 {
    (argb >> 16 & 255) as u8
}

/// Returns the green component of a color in ARGB format.
pub fn green_from_argb(argb: u32) -> u8 {
    (argb >> 8 & 255) as u8
}

/// Returns the blue component of a color in ARGB format.
pub fn blue_from_argb(argb: u32) -> u8 {
    (argb & 255) as u8
}

/// Returns whether a color in ARGB format is opaque.
pub fn is_opaque(argb: u32) -> bool {
    alpha_from_argb(argb) >= 255
}

/// Converts a color from ARGB to XYZ.
pub fn argb_from_xyz(x: f64, y: f64, z: f64) -> u32 {
    let matrix = XYZ_TO_SRGB;
    let linear_r = matrix[0][0] * x + matrix[0][1] * y + matrix[0][2] * z;
    let linear_g = matrix[1][0] * x + matrix[1][1] * y + matrix[1][2] * z;
    let linear_b = matrix[2][0] * x + matrix[2][1] * y + matrix[2][2] * z;
    let r = delinearized(linear_r);
    let g = delinearized(linear_g);
    let b = delinearized(linear_b);
    argb_from_rgb(r, g, b)
}

/// Converts a color from XYZ to ARGB.
pub fn xyz_from_argb(argb: u32) -> [f64; 3] {
    let r = linearized(red_from_argb(argb));
    let g = linearized(green_from_argb(argb));
    let b = linearized(blue_from_argb(argb));
    matrix_multiply(&[r, g, b], &SRGB_TO_XYZ)
}

/// Converts a color represented in Lab color space into an ARGB integer.
pub fn argb_from_lab(l: f64, a: f64, b: f64) -> u32 {
    let white_point = WHITE_POINT_D65;
    let fy = (l + 16.0) / 116.0;
    let fx = a / 500.0 + fy;
    let fz = fy - b / 200.0;
    let x_normalized = lab_invf(fx);
    let y_normalized = lab_invf(fy);
    let z_normalized = lab_invf(fz);
    let x = x_normalized * white_point[0];
    let y = y_normalized * white_point[1];
    let z = z_normalized * white_point[2];
    argb_from_xyz(x, y, z)
}

/// Converts a color from ARGB representation to L*a*b* representation.
///
/// - `argb`: the ARGB representation of a color
/// - Returns a Lab object representing the color
pub fn lab_from_argb(argb: u32) -> [f64; 3] {
    let linear_r = linearized(red_from_argb(argb));
    let linear_g = linearized(green_from_argb(argb));
    let linear_b = linearized(blue_from_argb(argb));
    let matrix = SRGB_TO_XYZ;
    let x = matrix[0][0] * linear_r + matrix[0][1] * linear_g + matrix[0][2] * linear_b;
    let y = matrix[1][0] * linear_r + matrix[1][1] * linear_g + matrix[1][2] * linear_b;
    let z = matrix[2][0] * linear_r + matrix[2][1] * linear_g + matrix[2][2] * linear_b;
    let white_point = WHITE_POINT_D65;
    let x_normalized = x / white_point[0];
    let y_normalized = y / white_point[1];
    let z_normalized = z / white_point[2];
    let fx = lab_f(x_normalized);
    let fy = lab_f(y_normalized);
    let fz = lab_f(z_normalized);
    let l = 116.0 * fy - 16.0;
    let a = 500.0 * (fx - fy);
    let b = 200.0 * (fy - fz);
    [l, a, b]
}

/// Converts an L* value to an ARGB representation.
///
/// - `lstar`: L* in L*a*b*
/// - Returns ARGB representation of grayscale color with lightness matching L*
pub fn argb_from_lstar(lstar: f64) -> u32 {
    let y = y_from_lstar(lstar);
    let component = delinearized(y);
    argb_from_rgb(component, component, component)
}

/// Computes the L* value of a color in ARGB representation.
///
/// - `argb`: ARGB representation of a color
/// - Returns L*, from L*a*b*, coordinate of the color
pub fn lstar_from_argb(argb: u32) -> f64 {
    let y = xyz_from_argb(argb)[1];
    116.0 * lab_f(y / 100.0) - 16.0
}

/// Converts an L* value to a Y value.
///
/// L* in L*a*b* and Y in XYZ measure the same quantity, luminance.
/// L* measures perceptual luminance, a linear scale. Y in XYZ
/// measures relative luminance, a logarithmic scale.
///
/// - `lstar`: L* in L*a*b*
/// - Returns Y in XYZ
pub fn y_from_lstar(lstar: f64) -> f64 {
    100.0 * lab_invf((lstar + 16.0) / 116.0)
}

/// Converts a Y value to an L* value.
///
/// L* in L*a*b* and Y in XYZ measure the same quantity, luminance.
/// L* measures perceptual luminance, a linear scale. Y in XYZ
/// measures relative luminance, a logarithmic scale.
///
/// - `y`: Y in XYZ
/// - Returns L* in L*a*b*
pub fn lstar_from_y(y: f64) -> f64 {
    lab_f(y / 100.0) * 116.0 - 16.0
}

/// Linearizes an RGB component.
///
/// - `rgb_component`: 0 <= rgb_component <= 255, represents R/G/B channel
/// - Returns 0.0 <= output <= 100.0, color channel converted to linear RGB space
pub fn linearized(rgb_component: u8) -> f64 {
    let normalized = rgb_component as f64 / 255.0;
    if normalized <= 0.040449936 {
        normalized / 12.92 * 100.0
    } else {
        ((normalized + 0.055) / 1.055).powf(2.4) * 100.0
    }
}

/// Delinearizes an RGB component.
///
/// - `rgb_component`: 0.0 <= rgb_component <= 100.0, represents linear R/G/B channel
/// - Returns 0 <= output <= 255, color channel converted to regular RGB space
pub fn delinearized(rgb_component: f64) -> u8 {
    let normalized = rgb_component / 100.0;
    let delinearized: f64;
    if normalized <= 0.0031308 {
        delinearized = normalized * 12.92;
    } else {
        delinearized = 1.055 * normalized.powf(1.0 / 2.4) - 0.055;
    }

    clamp_int(0, 255, (delinearized * 255.0).round() as i32) as u8
}

/// Returns the standard white point; white on a sunny day.
///
/// Returns: The white point
pub fn white_point_d65() -> [f64; 3] {
    WHITE_POINT_D65
}

/// RGBA component
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// Return RGBA from a given int32 color
///
/// - `argb`: ARGB representation of a int32 color.
/// - Returns: RGBA representation of a int32 color.
pub fn rgba_from_argb(argb: u32) -> Rgba {
    let r = red_from_argb(argb);
    let g = green_from_argb(argb);
    let b = blue_from_argb(argb);
    let a = alpha_from_argb(argb);
    Rgba { r, g, b, a }
}

/// Return int32 color from a given RGBA component
///
/// - `rgba`: RGBA representation of a int32 color.
/// - Returns: ARGB representation of a int32 color.
pub fn argb_from_rgba(rgba: &Rgba) -> u32 {
    (rgba.a as u32) << 24 | (rgba.r as u32) << 16 | (rgba.g as u32) << 8 | rgba.b as u32
}

fn lab_f(t: f64) -> f64 {
    const E: f64 = 216.0 / 24389.0;
    const KAPPA: f64 = 24389.0 / 27.0;
    if t > E {
        t.powf(1.0 / 3.0)
    } else {
        (KAPPA * t + 16.0) / 116.0
    }
}

fn lab_invf(ft: f64) -> f64 {
    const E: f64 = 216.0 / 24389.0;
    const KAPPA: f64 = 24389.0 / 27.0;
    let ft3 = ft * ft * ft;
    if ft3 > E {
        ft3
    } else {
        (116.0 * ft - 16.0) / KAPPA
    }
}
