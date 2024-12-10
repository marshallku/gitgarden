#[cfg(test)]
mod tests {
    use crate::utils::string::minify_xml;

    #[test]
    fn test_removing_line_breaks() {
        let xml = r#"
<svg>
<circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" />
</svg>
"#;

        let minified_xml = r#"<svg><circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" /></svg>"#;

        assert_eq!(minify_xml(xml), minified_xml);
    }

    #[test]
    fn test_removing_spaces() {
        let xml = r#"
<svg>
    <circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" />
</svg>
"#;

        let minified_xml = r#"<svg><circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" /></svg>"#;

        assert_eq!(minify_xml(xml), minified_xml);
    }

    #[test]
    fn test_removing_spaces_between_attributes() {
        let xml = r#"
           <svg>
                <circle                       cx="50"               cy="50"  r="40"   stroke="black" stroke-width="3" fill="red" />
            </svg>
        "#;

        let minified_xml = r#"<svg><circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" /></svg>"#;

        assert_eq!(minify_xml(xml), minified_xml);
    }

    #[test]
    fn test_removing_unnecessary_spaces() {
        let xml = r#"
           <svg>
                <def>
                    <style>
                        .red {
                            fill: red;
                        }
                    </style>
                </def>
                <circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" />
            </svg>
        "#;

        let minified_xml = r#"<svg><def><style>.red{fill:red;}</style></def><circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" /></svg>"#;

        assert_eq!(minify_xml(xml), minified_xml);
    }
}
