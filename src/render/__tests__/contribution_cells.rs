#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        api::languages::MostUsedLanguage,
        render::{contribution_cells::ContributionCells, objects::Objects, renderable::Renderable},
    };
    use chrono::NaiveDate;

    #[test]
    fn test_contribution_cells_render_empty() {
        let year = 2024;
        let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let weeks = 1;
        let commits = HashMap::new();
        let most_used_languages = vec![MostUsedLanguage {
            name: "Rust".to_string(),
            color: "#dea584".to_string(),
            percentage: 100.0,
        }];

        let cells = ContributionCells::new(year, start_date, weeks, commits, most_used_languages);
        let rendered = cells.render();

        let object_id = format!("#{}", Objects::Dirt.to_string());

        assert_eq!(rendered.matches("<use").count(), 7);
        assert_eq!(rendered.matches(&object_id).count(), 7);
    }

    #[test]
    fn test_contribution_cells_render_with_commits() {
        let year = 2023;
        let start_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let weeks = 1;
        let mut commits = HashMap::new();
        commits.insert("2023-01-01".to_string(), 1);
        commits.insert("2023-01-03".to_string(), 2);
        commits.insert("2023-01-05".to_string(), 3);
        commits.insert("2023-01-07".to_string(), 4);
        let most_used_languages = vec![MostUsedLanguage {
            name: "Rust".to_string(),
            color: "#dea584".to_string(),
            percentage: 100.0,
        }];

        let cells = ContributionCells::new(year, start_date, weeks, commits, most_used_languages);
        let rendered = cells.render();

        assert_eq!(rendered.matches("<use").count(), 7 + 4);
        assert!(rendered.contains("flower-1"));
        assert!(rendered.contains("flower-2"));
        assert!(rendered.contains("flower-3"));
        assert!(rendered.contains("flower-4"));
    }

    #[test]
    fn test_most_used_language_is_none() {
        let year = 2024;
        let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let weeks = 1;
        let commits = HashMap::new();
        let most_used_languages = vec![];

        let cells = ContributionCells::new(year, start_date, weeks, commits, most_used_languages);
        let rendered = cells.render();

        assert_eq!(rendered.matches("<use").count(), 7);
        assert_eq!(rendered.matches("<rect").count(), 0);
    }
}
