#[cfg(test)]
mod tests {
    use std::f64;

    use crate::utils::coordinate::{generate_coordinate, is_in_rectangles, Rectangle};

    #[test]
    fn test_random_generation() {
        let (x1, y1) = generate_coordinate("test1", (0.0, 1.0), (0.0, 1.0), None).unwrap();
        let (x2, y2) = generate_coordinate("test2", (0.0, 1.0), (0.0, 1.0), None).unwrap();

        assert_ne!(
            (x1, y1),
            (x2, y2),
            "Different inputs should generate different coordinates"
        );
    }

    #[test]
    fn test_deterministic_output() {
        let input = "same_input";
        let (x1, y1) = generate_coordinate(input, (0.0, 1.0), (0.0, 1.0), None).unwrap();
        let (x2, y2) = generate_coordinate(input, (0.0, 1.0), (0.0, 1.0), None).unwrap();

        assert_eq!(
            (x1, y1),
            (x2, y2),
            "Same input should generate same coordinates"
        );
    }

    #[test]
    fn test_range_bounds() {
        let (x, y) = generate_coordinate("test", (0.0, 10.0), (-5.0, 5.0), None).unwrap();

        assert!(
            x >= 0.0 && x < 10.0,
            "X should be within the specified range"
        );
        assert!(
            y >= -5.0 && y < 5.0,
            "Y should be within the specified range"
        );
    }

    #[test]
    #[should_panic]
    fn test_zero_range() {
        generate_coordinate("test", (5.0, 5.0), (10.0, 10.0), None);
    }

    #[test]
    #[should_panic]
    fn test_reversed_range() {
        generate_coordinate("test", (10.0, 0.0), (5.0, -5.0), None);
    }

    #[test]
    #[should_panic]
    fn test_max_range() {
        generate_coordinate("test", (f64::MIN, f64::MAX), (f64::MIN, f64::MAX), None);
    }

    #[test]
    fn test_large_ranges() {
        let (x, y) = generate_coordinate("test", (-1e300, 1e300), (-1e300, 1e300), None).unwrap();

        assert!(x.is_finite(), "X should be finite even with large ranges");
        assert!(y.is_finite(), "Y should be finite even with large ranges");
        assert!(
            x >= -1e300 && x < 1e300,
            "X should be within the specified large range"
        );
        assert!(
            y >= -1e300 && y < 1e300,
            "Y should be within the specified large range"
        );
    }

    #[test]
    fn test_different_types() {
        let (x1, y1) = generate_coordinate("string_input", (0.0, 1.0), (0.0, 1.0), None).unwrap();
        let (x2, y2) = generate_coordinate(42_u32, (0.0, 1.0), (0.0, 1.0), None).unwrap();
        let (x3, y3) = generate_coordinate(vec![1, 2, 3], (0.0, 1.0), (0.0, 1.0), None).unwrap();

        assert!(
            x1 != x2 || y1 != y2,
            "Different types should likely produce different results"
        );
        assert!(
            x1 != x3 || y1 != y3,
            "Different types should likely produce different results"
        );
        assert!(
            x2 != x3 || y2 != y3,
            "Different types should likely produce different results"
        );
    }
}
