use std::fmt;

pub type ColorComp = f32;

/// A color with the components of red, green and blue, all between the values of 0.0 and 1.0
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color3 {
    pub r: ColorComp,
    pub g: ColorComp,
    pub b: ColorComp,
}

impl Color3 {
    /// Creates a new color, with parameters all between the value of 0.0 and 1.0
    /// # Arguements
    /// - `r`: red
    /// - `g`: green
    /// - `b`: blue
    /// # Returns
    /// Either:
    /// - `None` when any of the components are out of range
    /// - `Some`: a color
    pub fn new(r: ColorComp, g: ColorComp, b: ColorComp) -> Option<Self> {
        if !(0.0..=1.0).contains(&r) || !(0.0..=1.0).contains(&g) || !(0.0..=1.0).contains(&b) {
            // values need to be between 0.0 and 1.0
            return None;
        }

        Some(Self { r, g, b })
    }

    /// Create a new color from RGB color space
    /// # Arguementts
    /// - `r`: red
    /// - `g`: green
    /// - `b`: blue
    /// # Returns
    /// A color
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as ColorComp / 255.0,
            g: g as ColorComp / 255.0,
            b: b as ColorComp / 255.0,
        }
    }
}

impl Default for Color3 {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

impl fmt::Display for Color3 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "color3({}, {}, {})", self.r, self.g, self.b)
    }
}
