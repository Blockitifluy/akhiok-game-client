//! Defines datatypes for colors. Stores:
//! - `Color3`: *RGB*
use std::{error::Error, fmt};

/// The floating point type used for a color's components
pub type ColorComp = f32;

/// A color with the components of red, green and blue, all between the values of 0.0 and 1.0
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color3 {
    /// Red component of the color
    pub r: ColorComp,
    /// Green component of the color
    pub g: ColorComp,
    /// Blue component of the color
    pub b: ColorComp,
}
impl Color3 {
    /// A pure white color
    pub const fn white() -> Color3 {
        Color3 {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    /// A pure black color
    pub const fn black() -> Color3 {
        Color3 {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    /// A pure red color
    pub const fn red() -> Color3 {
        Color3 {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        }
    }

    /// A pure green color
    pub const fn green() -> Color3 {
        Color3 {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        }
    }

    /// A pure blue color
    pub const fn blue() -> Color3 {
        Color3 {
            r: 0.0,
            g: 0.0,
            b: 1.0,
        }
    }

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

    /// Converts the color to RGB color space.
    /// # Returns
    /// A tuple of (r, g, b)
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        )
    }

    /// Converts the color to hex color code.
    /// # Returns
    /// A hex code (formated in 0xRRGGBB)
    pub fn to_hex(&self) -> u32 {
        let (r, g, b) = self.to_rgb();
        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    /// Converts the color to HSV color space.
    /// # Returns
    /// A tuple of (h, s, v)
    pub fn to_hsv(&self) -> (i32, f32, f32) {
        let Color3 { r, g, b } = *self;

        let v = r.max(g.max(b));
        let min = r.min(g.min(b));

        let c = v - min;

        let s = { if v == 0.0 { 0.0 } else { c / v } };

        let h_r = {
            if v == min {
                0.0
            } else if v == r {
                (g - b) / c
            } else if v == g {
                (b - r) / c + 2.0
            } else if v == b {
                (r - g) / c + 4.0
            } else {
                unreachable!()
            }
        } as i32;

        let h = (h_r + 360) % 360;

        (h, s, v)
        // let min = r.min(g.min(b));
        // let v = r.min(g.min(b));
        // let c = v - min;
        // println!("{}", c);
        //
        // let s = { if v == 0.0 { 0.0 } else { c / v } };
        //
        // let h = {
        //     if c == 0.0 {
        //         0.0
        //     } else if r == v {
        //         (g - b) / c
        //     } else if g == v {
        //         2.0 + (b - r) / c
        //     } else if b == v {
        //         4.0 + (r - g) / c
        //     } else {
        //         unreachable!();
        //     }
        // } as i32;
        //
        // (h, s, v)
    }

    /// Creates a new color from RGB color space.
    /// # Arguements
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

    /// Creates a new color from hex color code.
    /// # Arguements
    /// - `hex`: the hex code (should be formated 0xRRGGBB)
    /// # Returns
    /// A color
    pub fn from_hex(hex: u32) -> Self {
        let r: u8 = ((hex >> 16) & 255) as u8;
        let g: u8 = ((hex >> 8) & 255) as u8;
        let b: u8 = (hex & 255) as u8;

        Self::from_rgb(r, g, b)
    }

    /// Creates a new color from HSV color space.
    /// # Arguements
    /// - `hue`: the hue (from 0 to 360)
    /// - `sat`: the satruation (from 0.0 to 1.0)
    /// - `val`: the value (from 0.0 to 1.0)
    /// # Returns
    /// A result, either:
    /// - `Color3`,
    /// - An error message
    pub fn from_hsv(hue: i32, sat: f32, val: f32) -> Result<Self, HSVConvertErr> {
        if !(0..360).contains(&hue) {
            return Err(HSVConvertErr::HueOutOfRange);
        }

        if !(0.0..=1.0).contains(&sat) {
            return Err(HSVConvertErr::SatruationOutOfRange);
        }

        if !(0.0..=1.0).contains(&val) {
            return Err(HSVConvertErr::ValueOutOfRange);
        }

        let c = val * sat;
        let h = hue / 60;
        let x = c * (1.0 - ((h as f32 % 2.0) - 1.0).abs());

        let (r_q, g_q, b_q) = {
            if (0..1).contains(&h) {
                (c, x, 0.0)
            } else if (1..2).contains(&h) {
                (x, c, 0.0)
            } else if (2..3).contains(&h) {
                (0.0, c, x)
            } else if (3..4).contains(&h) {
                (0.0, x, c)
            } else if (4..5).contains(&h) {
                (x, 0.0, c)
            } else if (5..6).contains(&h) {
                (c, 0.0, x)
            } else {
                unreachable!()
            }
        };

        let m = val - c;

        Ok(Self::new(r_q + m, g_q + m, b_q + m).unwrap())
    }
}

/// An error thrown inside HSV color space conversion.
/// # Used in
/// - `Color3.from_hsv`
#[derive(Debug)]
pub enum HSVConvertErr {
    /// When the hue is not in the range of 0 <= hue < 360.
    HueOutOfRange,
    /// When the satruation is not in the range of 0.0 <= satruation <= 1.0.
    SatruationOutOfRange,
    /// When the value is not in the range of 0.0 <= value <= 1.0/
    ValueOutOfRange,
}

impl fmt::Display for HSVConvertErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error when converting HSV color space: {self:?}")
    }
}

impl Error for HSVConvertErr {}

impl Default for Color3 {
    fn default() -> Self {
        Self::white()
    }
}

impl fmt::Display for Color3 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "color3({}, {}, {})", self.r, self.g, self.b)
    }
}
