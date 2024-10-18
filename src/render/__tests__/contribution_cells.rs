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
}
