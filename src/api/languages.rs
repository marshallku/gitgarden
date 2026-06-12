use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::github::github_graphql_request;

use super::structures::{GithubGraphQLError, GithubGraphQLResponse, ERROR_TYPE_REQUEST, ERROR_TYPE_RESPONSE};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LanguageData {
    pub user: Option<User>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub repositories: Repositories,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Repositories {
    pub nodes: Vec<Repository>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Repository {
    pub name: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub languages: Languages,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Languages {
    pub edges: Vec<LanguageEdge>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LanguageEdge {
    pub size: i32,
    pub node: LanguageNode,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LanguageNode {
    pub color: String,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MostUsedLanguage {
    pub color: String,
    pub name: String,
    pub percentage: f32,
}

pub async fn get_most_used_languages(
    user_name: &str,
    year: i32,
    token: &str,
) -> Result<Vec<MostUsedLanguage>, Vec<GithubGraphQLError>> {
    let query = include_str!("schemas/most_used_languages.gql");

    let headers: HashMap<&str, &str> = HashMap::new();

    let data = json!({
        "variables": {
            "login": user_name
        }
    });

    let response = match github_graphql_request(query, &headers, data, token).await {
        Ok(response) => response,
        Err(error) => {
            println!("Error: {:?}", error);
            return Err(vec![GithubGraphQLError {
                error_type: ERROR_TYPE_REQUEST.to_string(),
                locations: vec![],
                message: error.to_string(),
                path: vec![],
            }]);
        }
    };

    let parsed_response: Result<GithubGraphQLResponse<LanguageData>, _> =
        serde_json::from_value(response);
    let user_data = match parsed_response {
        Ok(GithubGraphQLResponse {
            data: Some(LanguageData { user: Some(user) }),
            errors: None,
        }) => user,
        // GraphQL may return both data and errors (e.g. data.user = null plus
        // a NOT_FOUND error). Preserve genuine errors so they stay cacheable.
        Ok(GithubGraphQLResponse {
            errors: Some(errors),
            ..
        }) => return Err(errors),
        _ => {
            return Err(vec![GithubGraphQLError {
                error_type: ERROR_TYPE_RESPONSE.to_string(),
                locations: vec![],
                message: "Unexpected response".to_string(),
                path: vec![],
            }])
        }
    };

    let nodes = user_data.repositories.nodes;
    let mut language_totals: HashMap<String, i32> = HashMap::new();
    let mut total_size = 0;

    // Sum up sizes for each language across all repositories
    for repo in nodes.clone() {
        if !repo.updated_at.starts_with(&year.to_string()) {
            continue;
        }

        for edge in &repo.languages.edges {
            *language_totals.entry(edge.node.name.clone()).or_insert(0) += edge.size;
            total_size += edge.size;
        }
    }

    // Calculate percentages and create MostUsedLanguage structs
    let calculated_result = language_totals
        .iter()
        .map(|(name, &size)| {
            let percentage = (size as f32 / total_size as f32) * 100.0;

            let color = nodes
                .iter()
                .find_map(|repo| {
                    repo.languages
                        .edges
                        .iter()
                        .find(|edge| edge.node.name == *name)
                        .map(|edge| edge.node.color.clone())
                })
                .unwrap_or_default();

            MostUsedLanguage {
                color,
                name: name.clone(),
                percentage,
            }
        })
        .collect();

    Ok(calculated_result)
}
