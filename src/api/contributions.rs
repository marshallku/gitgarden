use std::collections::HashMap;
use tl::{parse, Attributes, ParserOptions};

fn parse_attribute_to_utf8_str<'a>(attributes: &'a Attributes, key: &'a str) -> Option<&'a str> {
    attributes
        .get(key)
        .flatten()
        .and_then(|value| value.try_as_utf8_str())
}

fn parse_commit_from_string(
    data: &str,
) -> Result<HashMap<String, u32>, Box<dyn std::error::Error>> {
    let mut commits_by_day = HashMap::with_capacity(366);
    let document = parse(data, ParserOptions::default())?;

    document
        .nodes()
        .iter()
        .filter_map(|node| node.as_tag())
        .filter(|tag| tag.name() == "td")
        .for_each(|td| {
            let attributes = td.attributes();

            if let (Some(date), Some(level)) = (
                parse_attribute_to_utf8_str(attributes, "data-date"),
                parse_attribute_to_utf8_str(attributes, "data-level"),
            ) {
                if let Ok(level) = level.parse::<u32>() {
                    if level > 0 {
                        commits_by_day.insert(date.to_string(), level);
                    }
                }
            }
        });

    Ok(commits_by_day)
}

pub async fn get_daily_commits(user_name: &str, year: i32) -> Result<HashMap<String, u32>, String> {
    let from = format!("{}-01-01", year);
    let to = format!("{}-12-31", year);
    let query = format!("?from={}&to={}", from, to);
    let url = format!(
        "https://github.com/users/{}/contributions{}",
        user_name, query
    );

    let response = match reqwest::get(&url).await {
        Ok(response) => response,
        Err(error) => {
            println!("Error: {:?}", error);
            return Err(error.to_string());
        }
    };
    let body = match response.text().await {
        Ok(body) => body,
        Err(error) => {
            println!("Error: {:?}", error);
            return Err(error.to_string());
        }
    };
    let commits = match parse_commit_from_string(&body) {
        Ok(commits) => commits,
        Err(error) => {
            println!("Error: {:?}", error);
            return Err(error.to_string());
        }
    };

    Ok(commits)
}
