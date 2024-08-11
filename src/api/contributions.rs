use scraper::{Html, Selector};
use std::collections::HashMap;

fn parse_commit_from_string(
    data: &str,
) -> Result<HashMap<String, u32>, Box<dyn std::error::Error>> {
    let mut commits_by_day = HashMap::new();
    let document = Html::parse_document(data);
    let td_selector = Selector::parse("td").unwrap();

    for td in document.select(&td_selector) {
        let date = td.value().attr("data-date");
        let level = td.value().attr("data-level");

        match (date, level) {
            (Some(date), Some(level)) => {
                if let Ok(level) = level.parse::<u32>() {
                    if level > 0 {
                        commits_by_day.insert(date.to_string(), level);
                    }
                }
            }
            _ => continue,
        }
    }

    Ok(commits_by_day)
}

pub async fn get_daily_commits(
    user_name: &str,
    year: i32,
) -> Result<HashMap<String, u32>, Box<dyn std::error::Error>> {
    let from = format!("{}-01-01", year);
    let to = format!("{}-12-31", year);
    let query = format!("?from={}&to={}", from, to);
    let url = format!(
        "https://github.com/users/{}/contributions{}",
        user_name, query
    );

    let response = reqwest::get(&url).await?.text().await?;
    let commits = parse_commit_from_string(&response)?;

    Ok(commits)
}
