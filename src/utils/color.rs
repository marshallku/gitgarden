use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, thiserror::Error)]
pub enum RgbError {
    #[error("Invalid hex color format. Use #RRGGBB.")]
    InvalidFormat,
    #[error("Invalid hex value.")]
    InvalidHexValue,
}

impl Rgb {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Rgb { red, green, blue }
    }

    pub fn from_hex(hex: &str) -> Result<Self, RgbError> {
        if hex.len() != 7 || !hex.starts_with('#') {
            return Err(RgbError::InvalidFormat);
        }

        let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| RgbError::InvalidHexValue)?;
        let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| RgbError::InvalidHexValue)?;
        let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| RgbError::InvalidHexValue)?;

        Ok(Rgb::new(r, g, b))
    }

    /// Interpolates between two RGB colors based on percentage.
    pub fn interpolate(&self, color: &Rgb, percentage: f32) -> Self {
        let ratio = percentage / 100.0;
        let r = (self.red as f32 + (color.red as f32 - self.red as f32) * ratio).round() as u8;
        let g =
            (self.green as f32 + (color.green as f32 - self.green as f32) * ratio).round() as u8;
        let b = (self.blue as f32 + (color.blue as f32 - self.blue as f32) * ratio).round() as u8;

        Rgb::new(r, g, b)
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}
