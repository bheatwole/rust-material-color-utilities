// Utility methods for hexadecimal representations of colors.
use super::color_utils::*;

/// Returns the hex string representing a color.
///
/// - `argb`: ARGB representation of a color.
/// - Returns: Hex string representing color, ex. #ff0000 for red.
pub fn hex_from_argb(argb: u32) -> String {
    let r = red_from_argb(argb);
    let g = green_from_argb(argb);
    let b = blue_from_argb(argb);

    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

/// Returns the ARGB representation of a color from a hex string.
///
/// - `hex`: String representing color as hex code. Accepts strings with or without leading #,
///          and string representing the color using 3, 6, or 8 hex characters.
/// - Returns: ARGB representation of color.
pub fn argb_from_hex(hex: &str) -> u32 {
    let hex = hex.trim_start_matches('#');
    let (r, g, b) = match hex.len() {
        3 => (
            parse_int_hex(&hex[0..1].repeat(2)),
            parse_int_hex(&hex[1..2].repeat(2)),
            parse_int_hex(&hex[2..3].repeat(2)),
        ),
        6 => (
            parse_int_hex(&hex[0..2]),
            parse_int_hex(&hex[2..4]),
            parse_int_hex(&hex[4..6]),
        ),
        8 => (
            parse_int_hex(&hex[2..4]),
            parse_int_hex(&hex[4..6]),
            parse_int_hex(&hex[6..8]),
        ),
        _ => panic!("unexpected hex {}", hex),
    };

    ((255 << 24) | ((r & 0x0ff) << 16) | ((g & 0x0ff) << 8) | (b & 0x0ff)) as u32
}

fn parse_int_hex(value: &str) -> u32 {
    u32::from_str_radix(value, 16).unwrap()
}
