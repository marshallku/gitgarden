use axum::http::HeaderValue;
use reqwest::{
    header::{HeaderMap, HeaderName, ACCEPT, AUTHORIZATION, USER_AGENT},
    Client, Error,
};
use serde_json::{json, Value};
use std::collections::HashMap;

pub async fn github_graphql_request(
    query: &str,
    headers: &HashMap<&str, &str>,
    data: Value,
    token: &str,
) -> Result<Value, Error> {
    let client = Client::new();

    let mut request_headers = HeaderMap::new();
    request_headers.insert(
        AUTHORIZATION,
        format!("token {}", token).parse().unwrap_or(HeaderValue::from_static("")),
    );
    request_headers.insert(ACCEPT, "*/*".parse().unwrap_or(HeaderValue::from_static("")));
    request_headers.insert(USER_AGENT, "reqwest".parse().unwrap_or(HeaderValue::from_static("")));

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
