use crate::palettes::CorePalette;

pub struct Scheme {
    pub primary: u32,
    pub on_primary: u32,
    pub primary_container: u32,
    pub on_primary_container: u32,
    pub secondary: u32,
    pub on_secondary: u32,
    pub secondary_container: u32,
    pub on_secondary_container: u32,
    pub tertiary: u32,
    pub on_tertiary: u32,
    pub tertiary_container: u32,
    pub on_tertiary_container: u32,
    pub error: u32,
    pub on_error: u32,
    pub error_container: u32,
    pub on_error_container: u32,
    pub surface_dim: u32,
    pub surface: u32,
    pub surface_bright: u32,
    pub surface_container_lowest: u32,
    pub surface_container_low: u32,
    pub surface_container: u32,
    pub surface_container_high: u32,
    pub surface_container_highest: u32,
    pub on_surface: u32,
    pub on_surface_variant: u32,
    pub outline: u32,
    pub outline_variant: u32,
    pub inverse_surface: u32,
    pub inverse_on_surface: u32,
    pub inverse_primary: u32,
    pub scrim: u32,
    pub shadow: u32,
}

impl Scheme {
    /// Light scheme from core palette
    pub fn light_from_core_palette(core: CorePalette) -> Scheme {
        Scheme {
            primary: core.a1.tone(40),
            on_primary: core.a1.tone(100),
            primary_container: core.a1.tone(90),
            on_primary_container: core.a1.tone(10),
            secondary: core.a2.tone(40),
            on_secondary: core.a2.tone(100),
            secondary_container: core.a2.tone(90),
            on_secondary_container: core.a2.tone(10),
            tertiary: core.a3.tone(40),
            on_tertiary: core.a3.tone(100),
            tertiary_container: core.a3.tone(90),
            on_tertiary_container: core.a3.tone(10),
            error: core.error.tone(40),
            on_error: core.error.tone(100),
            error_container: core.error.tone(90),
            on_error_container: core.error.tone(10),
            surface_dim: core.n1.tone(87),
            surface: core.n1.tone(98),
            surface_bright: core.n1.tone(98),
            surface_container_lowest: core.n1.tone(100),
            surface_container_low: core.n1.tone(96),
            surface_container: core.n1.tone(94),
            surface_container_high: core.n1.tone(92),
            surface_container_highest: core.n1.tone(90),
            on_surface: core.n1.tone(10),
            on_surface_variant: core.n2.tone(30),
            outline: core.n2.tone(50),
            outline_variant: core.n2.tone(80),
            shadow: core.n1.tone(0),
            scrim: core.n1.tone(0),
            inverse_surface: core.n1.tone(20),
            inverse_on_surface: core.n1.tone(95),
            inverse_primary: core.a1.tone(80),
        }
    }

    /// Dark scheme from core palette
    pub fn dark_from_core_palette(core: CorePalette) -> Scheme {
        Scheme {
            primary: core.a1.tone(80),
            on_primary: core.a1.tone(20),
            primary_container: core.a1.tone(30),
            on_primary_container: core.a1.tone(90),
            secondary: core.a2.tone(80),
            on_secondary: core.a2.tone(20),
            secondary_container: core.a2.tone(30),
            on_secondary_container: core.a2.tone(90),
            tertiary: core.a3.tone(80),
            on_tertiary: core.a3.tone(20),
            tertiary_container: core.a3.tone(30),
            on_tertiary_container: core.a3.tone(90),
            error: core.error.tone(80),
            on_error: core.error.tone(20),
            error_container: core.error.tone(30),
            on_error_container: core.error.tone(90),
            surface_dim: core.n1.tone(6),
            surface: core.n1.tone(6),
            surface_bright: core.n1.tone(24),
            surface_container_lowest: core.n1.tone(4),
            surface_container_low: core.n1.tone(10),
            surface_container: core.n1.tone(12),
            surface_container_high: core.n1.tone(17),
            surface_container_highest: core.n1.tone(22),
            on_surface: core.n1.tone(90),
            on_surface_variant: core.n2.tone(80),
            outline: core.n2.tone(60),
            outline_variant: core.n2.tone(30),
            shadow: core.n1.tone(0),
            scrim: core.n1.tone(0),
            inverse_surface: core.n1.tone(90),
            inverse_on_surface: core.n1.tone(20),
            inverse_primary: core.a1.tone(40),
        }
    }

    /// Returns a light Material color scheme based on the color's hue.
    ///
    /// # Arguments
    ///
    /// * `argb` - ARGB representation of a color.
    ///
    /// # Returns
    ///
    /// A light Material color scheme.
    pub fn light(argb: u32) -> Scheme {
        Scheme::light_from_core_palette(CorePalette::of(argb))
    }

    /// Returns a dark Material color scheme based on the color's hue.
    ///
    /// # Arguments
    ///
    /// * `argb` - ARGB representation of a color.
    ///
    /// # Returns
    ///
    /// A dark Material color scheme.
    pub fn dark(argb: u32) -> Scheme {
        Scheme::dark_from_core_palette(CorePalette::of(argb))
    }

    /// Returns a light Material content color scheme based on the color's hue.
    ///
    /// # Arguments
    ///
    /// * `argb` - ARGB representation of a color.
    ///
    /// # Returns
    ///
    /// A light Material content color scheme.
    pub fn light_content(argb: u32) -> Scheme {
        Scheme::light_from_core_palette(CorePalette::content_of(argb))
    }

    /// Returns a dark Material content color scheme based on the color's hue.
    ///
    /// # Arguments
    ///
    /// * `argb` - ARGB representation of a color.
    ///
    /// # Returns
    ///
    /// A dark Material content color scheme.
    pub fn dark_content(argb: u32) -> Scheme {
        Scheme::dark_from_core_palette(CorePalette::content_of(argb))
    }
}
