#[cfg(test)]
mod tests {
    use std::f64;

    use crate::utils::coordinate::{generate_coordinate, is_in_rectangle, Rectangle};

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

    #[test]
    fn test_dead_zone_exclusion() {
        let dead_zone = Rectangle {
            x1: 0.0,
            y1: 0.0,
            x2: 0.5,
            y2: 0.5,
        };

        for _ in 0..100 {
            // Run multiple times to ensure consistency
            if let Some((x, y)) =
                generate_coordinate("test", (0.0, 1.0), (0.0, 1.0), Some(&dead_zone))
            {
                assert!(
                    !is_in_rectangle(x, y, &dead_zone),
                    "Generated coordinate ({}, {}) should not be in dead zones",
                    x,
                    y
                );
            } else {
                panic!("Failed to generate coordinate outside dead zones");
            }
        }
    }

    #[test]
    fn test_dead_zone_covering_entire_range() {
        let dead_zone = Rectangle {
            x1: 0.0,
            y1: 0.0,
            x2: 1.0,
            y2: 1.0,
        };

        let result = generate_coordinate("test", (0.0, 1.0), (0.0, 1.0), Some(&dead_zone));
        assert!(
            result.is_none(),
            "Should return None when dead zone covers entire range"
        );
    }

    #[test]
    fn test_dead_zone_with_small_valid_area() {
        let dead_zone = Rectangle {
            x1: 0.0,
            y1: 0.0,
            x2: 0.99,
            y2: 0.99,
        };

        let result = generate_coordinate("test", (0.0, 1.0), (0.0, 1.0), Some(&dead_zone));
        assert!(
            result.is_some(),
            "Should be able to find a valid coordinate in a small area"
        );
        if let Some((x, y)) = result {
            assert!(
                x > 0.99 || y > 0.99,
                "Coordinate should be in the small valid area"
            );
        }
    }

    #[test]
    fn test_dead_zone_at_edges() {
        let dead_zones = vec![
            Rectangle {
                x1: 0.0,
                y1: 0.0,
                x2: 0.1,
                y2: 1.0,
            },
            Rectangle {
                x1: 0.9,
                y1: 0.0,
                x2: 1.0,
                y2: 1.0,
            },
        ];

        for dead_zone in dead_zones {
            for _ in 0..100 {
                // Run multiple times to ensure consistency
                if let Some((x, _y)) =
                    generate_coordinate("test", (0.0, 1.0), (0.0, 1.0), Some(&dead_zone))
                {
                    assert!(
                        x > 0.1 && x < 0.9,
                        "X coordinate should be between dead zones"
                    );
                } else {
                    panic!("Failed to generate coordinate outside dead zones");
                }
            }
        }
    }
}
