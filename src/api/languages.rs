use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::github::github_graphql_request;

use super::structures::{GithubGraphQLError, GithubGraphQLResponse};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Root {
    pub data: LanguageData,
}

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

#[allow(dead_code)]
pub async fn get_most_used_languages(
    user_name: &str,
    token: &str,
) -> Result<Vec<MostUsedLanguage>, Vec<GithubGraphQLError>> {
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
    }
    "#;

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
                error_type: "RequestError".to_string(),
                locations: vec![],
                message: error.to_string(),
                path: vec![],
            }]);
        }
    };

    let response: GithubGraphQLResponse<LanguageData> = serde_json::from_value(response).unwrap();

    let response = match response {
        GithubGraphQLResponse {
            data: Some(data),
            errors: None,
        } => data.user.unwrap(),
        _ => {
            return Err(vec![GithubGraphQLError {
                error_type: "ResponseError".to_string(),
                locations: vec![],
                message: "Unexpected response".to_string(),
                path: vec![],
            }])
        }
    };

    let nodes = response.repositories.nodes;
    let mut language_totals: HashMap<String, i32> = HashMap::new();
    let mut total_size = 0;

    // Sum up sizes for each language across all repositories
    for repo in nodes.clone() {
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
