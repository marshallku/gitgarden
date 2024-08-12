use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    data: Option<Data>,
    errors: Option<Vec<Error>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    user: Option<User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "starredRepositories")]
    starred_repositories: StarredRepositories,
    #[serde(rename = "contributionsCollection")]
    contributions_collection: ContributionsCollection,
    #[serde(rename = "pullRequests")]
    pull_requests: PullRequests,
    issues: Issues,
    #[serde(rename = "repositoriesContributedTo")]
    repositories_contributed_to: RepositoriesContributedTo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StarredRepositories {
    #[serde(rename = "totalCount")]
    total_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContributionsCollection {
    #[serde(rename = "totalCommitContributions")]
    total_commit_contributions: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequests {
    #[serde(rename = "totalCount")]
    total_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Issues {
    #[serde(rename = "totalCount")]
    total_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepositoriesContributedTo {
    #[serde(rename = "totalCount")]
    total_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    #[serde(rename = "type")]
    error_type: String,
    path: Vec<String>,
    locations: Vec<Location>,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    line: i32,
    column: i32,
}

use crate::utils::github::github_graphql_request;

pub async fn get_stats(user_name: String, token: String) -> Result<User, Vec<Error>> {
    let query = r#"
    query userInfo($login: String!) {
        user(login: $login) {
            repositories(ownerAffiliations: OWNER, isFork: false, first: 100) {
                nodes {
                    name
                    languages(first: 10, orderBy: {field: SIZE, direction: DESC}) {
                        edges {
                            size
                            node {
                                color
                                name
                            }
                        }
                    }
                }
            }
        }
    }"#;

    let headers: HashMap<&str, &str> = HashMap::new();

    let data = json!({
        "variables": {
            "login": user_name
        }
    });

    let response = github_graphql_request(query, &headers, data, &token)
        .await
        .unwrap();

    let response: ApiResponse = serde_json::from_value(response).unwrap();

    match response {
        ApiResponse {
            data: Some(data),
            errors: None,
        } => Ok(data.user.unwrap()),
        ApiResponse {
            data: None,
            errors: Some(errors),
        } => Err(errors),
        _ => panic!("Unexpected response"),
    }
}
