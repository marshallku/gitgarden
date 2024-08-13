use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::github::github_graphql_request;

use super::structures::{GithubGraphQLError, GithubGraphQLResponse};

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub user: Option<User>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "starredRepositories")]
    pub starred_repositories: StarredRepositories,
    #[serde(rename = "contributionsCollection")]
    pub contributions_collection: ContributionsCollection,
    #[serde(rename = "pullRequests")]
    pub pull_requests: PullRequests,
    pub issues: Issues,
    #[serde(rename = "repositoriesContributedTo")]
    pub repositories_contributed_to: RepositoriesContributedTo,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StarredRepositories {
    #[serde(rename = "totalCount")]
    pub total_count: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ContributionsCollection {
    #[serde(rename = "totalCommitContributions")]
    pub total_commit_contributions: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PullRequests {
    #[serde(rename = "totalCount")]
    pub total_count: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Issues {
    #[serde(rename = "totalCount")]
    pub total_count: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RepositoriesContributedTo {
    #[serde(rename = "totalCount")]
    pub total_count: i32,
}

pub async fn get_stats(user_name: String, token: String) -> Result<User, Vec<GithubGraphQLError>> {
    let query = r#"
    query UserStats($login: String!) {
        user(login: $login) {
            starredRepositories {
                totalCount
            }
            contributionsCollection {
                totalCommitContributions
            }
            pullRequests {
                totalCount
            }
            issues {
                totalCount
            }
            repositoriesContributedTo {
                totalCount
            }
        }
    }"#;

    let headers: HashMap<&str, &str> = HashMap::new();

    let data = json!({
        "variables": {
            "login": user_name
        }
    });

    let response = match github_graphql_request(query, &headers, data, &token).await {
        Ok(response) => response,
        Err(error) => {
            println!("Error: {:?}", error);
            return Err(vec![GithubGraphQLError {
                error_type: "RequestError".to_string(),
                locations: vec![],
                message: error.to_string(),
                path: vec![],
            }]);
        }
    };

    let response: GithubGraphQLResponse<Data> = serde_json::from_value(response).unwrap();

    match response {
        GithubGraphQLResponse {
            data: Some(data),
            errors: None,
        } => Ok(data.user.unwrap()),
        GithubGraphQLResponse {
            data: None,
            errors: Some(errors),
        } => Err(errors),
        _ => panic!("Unexpected response"),
    }
}
