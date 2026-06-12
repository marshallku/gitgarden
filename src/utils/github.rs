use axum::http::HeaderValue;
use reqwest::{
    header::{HeaderMap, HeaderName, ACCEPT, AUTHORIZATION, USER_AGENT},
    Client, Error,
};
use serde_json::{json, Value};
use std::collections::HashMap;

pub async fn github_graphql_request(
    client: &Client,
    query: &str,
    headers: &HashMap<&str, &str>,
    data: Value,
    token: &str,
) -> Result<Value, Error> {
    let mut request_headers = HeaderMap::new();
    request_headers.insert(
        AUTHORIZATION,
        format!("token {}", token)
            .parse()
            .unwrap_or(HeaderValue::from_static("")),
    );
    request_headers.insert(
        ACCEPT,
        "*/*".parse().unwrap_or(HeaderValue::from_static("")),
    );
    request_headers.insert(
        USER_AGENT,
        "reqwest".parse().unwrap_or(HeaderValue::from_static("")),
    );

    for (key, value) in headers {
        request_headers.insert(
            HeaderName::from_bytes(key.as_bytes()).unwrap_or(HeaderName::from_static("")),
            value.parse().unwrap_or(HeaderValue::from_static("")),
        );
    }

    let mut body = json!({
        "query": query
    });

    // Merge the additional data into the body
    if let Some(obj) = body.as_object_mut() {
        if let Some(data_obj) = data.as_object() {
            obj.extend(data_obj.clone());
        }
    }

    let response = client
        .post("https://api.github.com/graphql")
        .headers(request_headers)
        .json(&body)
        .send()
        .await?;

    let json_response: Value = response.json().await?;

    Ok(json_response)
}

/// GitHub usernames are 1-39 characters of alphanumerics or hyphens and
/// cannot start or end with a hyphen. Anything else never resolves upstream,
/// so reject it before building request URLs from it.
pub fn is_valid_github_username(user_name: &str) -> bool {
    if user_name.is_empty() || user_name.len() > 39 {
        return false;
    }

    if user_name.starts_with('-') || user_name.ends_with('-') {
        return false;
    }

    user_name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-')
}
