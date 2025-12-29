pub type ColorComp = f32;

#[derive(Clone, Copy, Debug)]
pub struct Color3 {
    pub r: ColorComp,
    pub g: ColorComp,
    pub b: ColorComp,
}

impl Color3 {
    pub fn new(r: ColorComp, g: ColorComp, b: ColorComp) -> Option<Self> {
        if (0.0 > r && r < 1.0) || (0.0 > g && g < 1.0) || (0.0 > b && b < 1.0) {
            // values need to be between 0.0 and 1.0
            return None;
        }

        Some(Self { r, g, b })
    }

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
