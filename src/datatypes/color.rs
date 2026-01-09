//! Defines datatypes for colors. Stores:
//! - `Color3`: *RGB*
use std::{fmt, str::FromStr};

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
        let c_max = r.max(g.max(b));
        let c_min = r.min(g.min(b));

        let delta = c_max - c_min;
        let val = c_max;

        let sat = { if c_max == 0.0 { 0.0 } else { delta / c_max } };

        let hue = {
            if delta == 0.0 {
                0
            } else if c_max == r {
                (60.0 * ((g - b) / delta % 6.0)) as i32
            } else if c_max == g {
                (60.0 * ((b - r) / delta + 2.0)) as i32
            } else {
                (60.0 * ((r - g) / delta + 4.0)) as i32
            }
        };

        (hue, sat, val)
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
    pub fn from_hsv(hue: i32, sat: f32, val: f32) -> Result<Self, String> {
        if !(0..360).contains(&hue) {
            return Err(String::from_str("hue not in range of 0 to 360").unwrap());
        }

        if !(0.0..1.0).contains(&sat) {
            return Err(String::from_str("satruation not in range of 0 to 1").unwrap());
        }

        if !(0.0..1.0).contains(&val) {
            return Err(String::from_str("value not in range of 0 to 1").unwrap());
        }

        let hue_f = hue as f32;

        let c: f32 = val * sat;
        let x: f32 = c * (1.0 - f32::abs((hue_f / 60.0 % 2.0) - 1.0));
        let m: f32 = val - c;
        let r: i32 = hue / 60;

        if !(0..=5).contains(&r) {
            return Err(String::from_str("hue out of range of 0 to 360").unwrap());
        }

        let (r_q, g_q, b_q);
        match r {
            0 => {
                r_q = c;
                g_q = x;
                b_q = 0.0;
            }
            1 => {
                r_q = x;
                g_q = c;
                b_q = 0.0;
            }
            2 => {
                r_q = 0.0;
                g_q = c;
                b_q = x;
            }
            3 => {
                r_q = 0.0;
                g_q = x;
                b_q = c;
            }
            4 => {
                r_q = x;
                g_q = 0.0;
                b_q = c;
            }
            5 => {
                r_q = c;
                g_q = 0.0;
                b_q = 0.0;
            }
            _ => panic!("hue was out of range even when multiple checks were in place"),
        }

        Ok(Self::new(r_q + m, g_q + m, b_q + m).unwrap())
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
