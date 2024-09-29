#[cfg(test)]
mod tests {
    use crate::utils::color::Rgb;

    #[test]
    fn test_new() {
        let color = Rgb::new(255, 128, 64);
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
    }

    #[test]
    fn test_from_hex_valid() {
        let color = Rgb::try_from("#FF8040").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 128);
        assert_eq!(color.blue, 64);
    }

    #[test]
    fn test_try_from_invalid() {
        assert!(Rgb::try_from("FF8040").is_err());
        assert!(Rgb::try_from("#FF804").is_err());
        assert!(Rgb::try_from("#FF80ZZ").is_err());
    }

    #[test]
    fn test_to_hex() {
        let color = Rgb::new(255, 128, 64);
        assert_eq!(format!("{}", color), "#FF8040");
    }

    #[test]
    fn test_interpolate() {
        let color1 = Rgb::new(0, 0, 0);
        let color2 = Rgb::new(255, 255, 255);

        let interpolated = color1.interpolate(&color2, 50.0);
        assert_eq!(interpolated.red, 128);
        assert_eq!(interpolated.green, 128);
        assert_eq!(interpolated.blue, 128);

        let interpolated = color1.interpolate(&color2, 25.0);
        assert_eq!(interpolated.red, 64);
        assert_eq!(interpolated.green, 64);
        assert_eq!(interpolated.blue, 64);

        let interpolated = color1.interpolate(&color2, 75.0);
        assert_eq!(interpolated.red, 191);
        assert_eq!(interpolated.green, 191);
        assert_eq!(interpolated.blue, 191);
    }
}
