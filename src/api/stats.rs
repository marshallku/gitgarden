use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::github::github_graphql_request;

use super::structures::{GithubGraphQLError, GithubGraphQLResponse};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserIdData {
    pub user: Option<UserId>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserId {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub user: Option<User>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub login: String,
    #[serde(rename = "contributionsCollection")]
    pub contributions_collection: ContributionsCollection,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ContributionsCollection {
    #[serde(rename = "totalCommitContributions")]
    pub total_commit_contributions: i32,
    #[serde(rename = "totalIssueContributions")]
    pub total_issue_contributions: i32,
    #[serde(rename = "totalPullRequestContributions")]
    pub total_pull_request_contributions: i32,
    #[serde(rename = "totalPullRequestReviewContributions")]
    pub total_pull_request_review_contributions: i32,
    #[serde(rename = "totalRepositoriesWithContributedCommits")]
    pub total_repositories_with_contributed_commits: i32,
    #[serde(rename = "totalRepositoriesWithContributedIssues")]
    pub total_repositories_with_contributed_issues: i32,
    #[serde(rename = "totalRepositoriesWithContributedPullRequests")]
    pub total_repositories_with_contributed_pull_requests: i32,
    #[serde(rename = "totalRepositoriesWithContributedPullRequestReviews")]
    pub total_repositories_with_contributed_pull_request_reviews: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Repository {
    #[serde(rename = "defaultBranchRef")]
    pub default_branch_ref: Option<DefaultBranchRef>,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DefaultBranchRef {
    pub target: Target,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Target {
    pub history: History,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct History {
    pub edges: Vec<Edge>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Edge {
    pub node: Node,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Node {
    pub message: String,
    pub author: Author,
    #[serde(rename = "committedDate")]
    pub committed_date: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Author {
    pub email: String,
}

#[allow(dead_code)]
pub async fn get_user_id(user_name: &str, token: &str) -> Result<String, Vec<GithubGraphQLError>> {
    let query = r#"
    query($login: String!) {
        user(login: $login) {
            id
        }
    }
    "#;

    let headers: HashMap<&str, &str> = HashMap::new();

    let data = json!({
        "variables": {
            "login": user_name,
        }
    });

    let response = match github_graphql_request(query, &headers, data, token).await {
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

    let response: GithubGraphQLResponse<UserIdData> = serde_json::from_value(response).unwrap();

    match response {
        GithubGraphQLResponse {
            data: Some(data),
            errors: None,
        } => Ok(data.user.unwrap().id),
        GithubGraphQLResponse {
            data: None,
            errors: Some(errors),
        } => Err(errors),
        _ => Err(vec![GithubGraphQLError {
            error_type: "ResponseError".to_string(),
            locations: vec![],
            message: format!("Invalid user ID: {}", user_name),
            path: vec!["unknown".to_string()],
        }]),
    }
}

pub async fn get_stats(
    user_name: &str,
    from: String,
    to: String,
    token: &str,
) -> Result<User, Vec<GithubGraphQLError>> {
    let query = r#"
    query($login: String!, $from: DateTime!, $to: DateTime!) {
        user(login: $login) {
            login contributionsCollection(from: $from, to: $to) {
                totalCommitContributions
                totalIssueContributions
                totalPullRequestContributions
                totalPullRequestReviewContributions
                totalRepositoriesWithContributedCommits
                totalRepositoriesWithContributedIssues
                totalRepositoriesWithContributedPullRequests
                totalRepositoriesWithContributedPullRequestReviews
            }
        } 
    }
    "#;

    let headers: HashMap<&str, &str> = HashMap::new();

    let data = json!({
        "variables": {
            "login": user_name,
            "from": from,
            "to": to,
        }
    });

    let response = match github_graphql_request(query, &headers, data, token).await {
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
