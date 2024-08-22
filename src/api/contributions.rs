use std::collections::HashMap;
use tl::{parse, ParserOptions};

fn parse_commit_from_string(
    data: &str,
) -> Result<HashMap<String, u32>, Box<dyn std::error::Error>> {
    let mut commits_by_day = HashMap::new();
    let document = parse(data, ParserOptions::default()).unwrap();
    let nodes = document
        .nodes()
        .iter()
        .filter(|node| node.as_tag().map_or(false, |tag| tag.name() == "td"));

    for td in nodes {
        let td = td.as_tag().unwrap();

        let attributes = td.attributes();

        let date = attributes
            .get("data-date")
            .flatten()
            .and_then(|date| date.try_as_utf8_str());
        let level = attributes
            .get("data-level")
            .flatten()
            .and_then(|level| level.try_as_utf8_str());

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
