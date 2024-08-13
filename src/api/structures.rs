use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubGraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GithubGraphQLError>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubGraphQLError {
    #[serde(rename = "type")]
    pub error_type: String,
    pub path: Vec<String>,
    pub locations: Vec<Location>,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub line: i32,
    pub column: i32,
}
