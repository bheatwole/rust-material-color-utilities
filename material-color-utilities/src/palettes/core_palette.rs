use crate::hct::Hct;

use super::tonal_palette::TonalPalette;

/// Set of colors to generate a `CorePalette` from
pub struct CorePaletteColors {
    pub primary: u32,
    pub secondary: Option<u32>,
    pub tertiary: Option<u32>,
    pub neutral: Option<u32>,
    pub neutral_variant: Option<u32>,
    pub error: Option<u32>,
}

/// An intermediate concept between the key color for a UI theme, and a full
/// color scheme. 5 sets of tones are generated, all except one use the same hue
/// as the key color, and all vary in chroma.
pub struct CorePalette {
    a1: TonalPalette,
    a2: TonalPalette,
    a3: TonalPalette,
    n1: TonalPalette,
    n2: TonalPalette,
    error: TonalPalette,
}

impl CorePalette {
    /// Create a `CorePalette` from an ARGB color representation
    ///
    /// # Arguments
    ///
    /// * `argb` - ARGB representation of a color
    pub fn of(argb: u32) -> CorePalette {
        CorePalette::new(argb, false)
    }

    /// Create a content `CorePalette` from an ARGB color representation
    ///
    /// # Arguments
    ///
    /// * `argb` - ARGB representation of a color
    pub fn content_of(argb: u32) -> CorePalette {
        CorePalette::new(argb, true)
    }

    /// Create a `CorePalette` from a set of colors
    ///
    /// # Arguments
    ///
    /// * `colors` - Set of colors
    pub fn from_colors(colors: CorePaletteColors) -> CorePalette {
        CorePalette::create_palette_from_colors(false, colors)
    }

    /// Create a content `CorePalette` from a set of colors
    ///
    /// # Arguments
    ///
    /// * `colors` - Set of colors
    pub fn content_from_colors(colors: CorePaletteColors) -> CorePalette {
        CorePalette::create_palette_from_colors(true, colors)
    }

    fn create_palette_from_colors(content: bool, colors: CorePaletteColors) -> CorePalette {
        let mut palette = CorePalette::new(colors.primary, content);
        if let Some(secondary) = colors.secondary {
            let p = CorePalette::new(secondary, content);
            palette.a2 = p.a1;
        }
        if let Some(tertiary) = colors.tertiary {
            let p = CorePalette::new(tertiary, content);
            palette.a3 = p.a1;
        }
        if let Some(error) = colors.error {
            let p = CorePalette::new(error, content);
            palette.error = p.a1;
        }
        if let Some(neutral) = colors.neutral {
            let p = CorePalette::new(neutral, content);
            palette.n1 = p.n1;
        }
        if let Some(neutral_variant) = colors.neutral_variant {
            let p = CorePalette::new(neutral_variant, content);
            palette.n2 = p.n2;
        }
        palette
    }

    fn new(argb: u32, is_content: bool) -> CorePalette {
        let hct = Hct::from_int(argb);
        let hue = hct.hue();
        let chroma = hct.chroma();
        let a1 = TonalPalette::from_hue_and_chroma(hue, if is_content { chroma } else { 48.0 });
        let a2 =
            TonalPalette::from_hue_and_chroma(hue, if is_content { chroma / 3.0 } else { 16.0 });
        let a3 = TonalPalette::from_hue_and_chroma(
            hue + 60.0,
            if is_content { chroma / 2.0 } else { 24.0 },
        );
        let n1 = TonalPalette::from_hue_and_chroma(
            hue,
            if is_content {
                chroma / 12f64.min(4.0)
            } else {
                4.0
            },
        );
        let n2 = TonalPalette::from_hue_and_chroma(
            hue,
            if is_content {
                chroma / 6f64.min(8.0)
            } else {
                8.0
            },
        );
        let error = TonalPalette::from_hue_and_chroma(25.0, 84.0);
        CorePalette {
            a1,
            a2,
            a3,
            n1,
            n2,
            error,
        }
    }
}
