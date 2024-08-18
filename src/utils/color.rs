#[derive(Debug, Clone)]
pub struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Rgb { red, green, blue }
    }

    pub fn from_hex(hex: &str) -> Result<Self, String> {
        if hex.len() != 7 || !hex.starts_with('#') {
            return Err("Invalid hex color format. Use #RRGGBB.".to_string());
        }

        let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Invalid hex value for red.")?;
        let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| "Invalid hex value for green.")?;
        let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| "Invalid hex value for blue.")?;

        Ok(Rgb::new(r, g, b))
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}
