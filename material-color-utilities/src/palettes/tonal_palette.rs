use std::collections::HashMap;

use crate::hct::Hct;

pub struct TonalPalette {
    cache: HashMap<u32, u32>,
    hue: f64,
    chroma: f64,
    key_color: Hct,
}

impl TonalPalette {
    /// Creates a TonalPalette from an ARGB representation of a color.
    ///
    /// # Arguments
    ///
    /// * `argb` - ARGB representation of a color
    ///
    /// # Returns
    ///
    /// Tones matching that color's hue and chroma.
    pub fn from_int(argb: u32) -> TonalPalette {
        let hct = Hct::from_int(argb);
        TonalPalette::from_hct(hct)
    }

    /// Creates a TonalPalette from an Hct.
    ///
    /// # Arguments
    ///
    /// * `hct` - Hct
    ///
    /// # Returns
    ///
    /// Tones matching that color's hue and chroma.
    pub fn from_hct(hct: Hct) -> TonalPalette {
        TonalPalette {
            cache: HashMap::new(),
            hue: hct.hue(),
            chroma: hct.chroma(),
            key_color: hct,
        }
    }

    /// Creates a TonalPalette from a hue and chroma.
    ///
    /// # Arguments
    ///
    /// * `hue` - HCT hue
    /// * `chroma` - HCT chroma
    ///
    /// # Returns
    ///
    /// Tones matching hue and chroma.
    pub fn from_hue_and_chroma(hue: f64, chroma: f64) -> TonalPalette {
        let key_color = TonalPalette::create_key_color(hue, chroma);
        TonalPalette {
            cache: HashMap::new(),
            hue,
            chroma,
            key_color,
        }
    }

    fn create_key_color(hue: f64, chroma: f64) -> Hct {
        const START_TONE: f64 = 50.0;
        let mut smallest_delta_hct = Hct::from_hct(hue, chroma, START_TONE);
        let mut smallest_delta = (smallest_delta_hct.chroma() - chroma).abs();

        for delta_int in 1..50 {
            let delta = delta_int as f64;
            if chroma.round() == smallest_delta_hct.chroma().round() {
                return smallest_delta_hct;
            }

            let hct_add = Hct::from_hct(hue, chroma, START_TONE + delta);
            let hct_add_delta = (hct_add.chroma() - chroma).abs();
            if hct_add_delta < smallest_delta {
                smallest_delta = hct_add_delta;
                smallest_delta_hct = hct_add;
            }

            let hct_subtract = Hct::from_hct(hue, chroma, START_TONE - delta);
            let hct_subtract_delta = (hct_subtract.chroma() - chroma).abs();
            if hct_subtract_delta < smallest_delta {
                smallest_delta = hct_subtract_delta;
                smallest_delta_hct = hct_subtract;
            }
        }

        smallest_delta_hct
    }

    /// Returns the ARGB representation of a color with the given tone.
    ///
    /// # Arguments
    ///
    /// * `tone` - HCT tone, measured from 0 to 100.
    ///
    /// # Returns
    ///
    /// ARGB representation of a color with that tone.
    pub fn tone(&mut self, tone: u32) -> u32 {
        debug_assert!(tone <= 100);

        *self
            .cache
            .entry(tone)
            .or_insert_with(|| Hct::from_hct(self.hue, self.chroma, tone as f64).to_int())
    }

    /// Returns the HCT representation of a color with the given tone.
    ///
    /// # Arguments
    ///
    /// * `tone` - HCT tone, measured from 0 to 100.
    ///
    /// # Returns
    ///
    /// HCT representation of a color with that tone.
    pub fn get_hct(&mut self, tone: u32) -> Hct {
        Hct::from_int(self.tone(tone))
    }
}
