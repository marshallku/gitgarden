use reqwest::{Client, Error};
use serde_json::{json, Value};
use std::collections::HashMap;

pub async fn github_graphql_request(
    query: &str,
    headers: &HashMap<&str, &str>,
    data: Value,
    token: &str,
) -> Result<Value, Error> {
    let client = Client::new();

    let mut request_headers = reqwest::header::HeaderMap::new();
    request_headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("token {}", token).parse().unwrap(),
    );
    request_headers.insert(reqwest::header::ACCEPT, "*/*".parse().unwrap());
    request_headers.insert(reqwest::header::USER_AGENT, "reqwest".parse().unwrap());

    for (key, value) in headers {
        request_headers.insert(
            reqwest::header::HeaderName::from_bytes(key.as_bytes()).unwrap(),
            value.parse().unwrap(),
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
