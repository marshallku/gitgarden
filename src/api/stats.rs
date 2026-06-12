use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::github::github_graphql_request;

use super::structures::{
    GithubGraphQLError, GithubGraphQLResponse, ERROR_TYPE_REQUEST, ERROR_TYPE_RESPONSE,
};

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

pub async fn get_stats(
    client: &reqwest::Client,
    user_name: &str,
    from: String,
    to: String,
    token: &str,
) -> Result<User, Vec<GithubGraphQLError>> {
    let query = include_str!("schemas/stats.gql");

    let headers: HashMap<&str, &str> = HashMap::new();

    let data = json!({
        "variables": {
            "login": user_name,
            "from": from,
            "to": to,
        }
    });

    let response = match github_graphql_request(client, query, &headers, data, token).await {
        Ok(response) => response,
        Err(error) => {
            tracing::error!("GitHub GraphQL request failed: {:?}", error);
            return Err(vec![GithubGraphQLError {
                error_type: ERROR_TYPE_REQUEST.to_string(),
                locations: vec![],
                message: error.to_string(),
                path: vec![],
            }]);
        }
    };

    let response: GithubGraphQLResponse<Data> = match serde_json::from_value(response) {
        Ok(parsed) => parsed,
        Err(error) => {
            return Err(vec![GithubGraphQLError {
                error_type: ERROR_TYPE_RESPONSE.to_string(),
                locations: vec![],
                message: error.to_string(),
                path: vec![],
            }])
        }
    };

    match response {
        GithubGraphQLResponse {
            data: Some(Data { user: Some(user) }),
            errors: None,
        } => Ok(user),
        // GraphQL may return both data and errors (e.g. data.user = null plus
        // a NOT_FOUND error). Preserve genuine errors so they stay cacheable.
        GithubGraphQLResponse {
            errors: Some(errors),
            ..
        } => Err(errors),
        _ => Err(vec![GithubGraphQLError {
            error_type: ERROR_TYPE_RESPONSE.to_string(),
            locations: vec![],
            message: format!("Invalid user: {}", user_name),
            path: vec!["unknown".to_string()],
        }]),
    }
}
