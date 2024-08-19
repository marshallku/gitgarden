use crate::{api::stats::ContributionsCollection, utils::coordinate::must_generate_coordinate};

use super::{objects::Objects, renderable::Renderable};

pub struct Grasses {
    user_name: String,
    width: u32,
    contributions: ContributionsCollection,
}

impl Grasses {
    pub fn new(user_name: &str, width: u32, contributions: &ContributionsCollection) -> Self {
        Self {
            user_name: user_name.to_string(),
            width,
            contributions: contributions.clone(),
        }
    }
}

impl Renderable for Grasses {
    fn render(&self) -> String {
        let x_max = self.width as f64 - 16.0;
        let y_max = 300.0;

        let grass_types = [
            (
                Objects::GrassOne,
                self.contributions.total_issue_contributions,
            ),
            (
                Objects::GrassTwo,
                self.contributions.total_pull_request_contributions,
            ),
            (
                Objects::GrassThree,
                self.contributions.total_pull_request_review_contributions,
            ),
            (
                Objects::GrassFour,
                self.contributions
                    .total_repositories_with_contributed_issues,
            ),
            (
                Objects::GrassFive,
                self.contributions
                    .total_repositories_with_contributed_pull_requests,
            ),
            (
                Objects::GrassSix,
                self.contributions
                    .total_repositories_with_contributed_pull_request_reviews,
            ),
        ];

        grass_types
            .iter()
            .enumerate()
            .flat_map(|(index, (grass_type, count))| {
                (0..*count).map(move |i| {
                    let (x, y) = must_generate_coordinate(
                        &format!("{}-grass-{}-{}", self.user_name, index + 1, i),
                        (0.0, x_max),
                        (0.0, y_max),
                        None,
                    );
                    format!(
                        r##"<use x="{}" y="{}" xlink:href="#{}" />"##,
                        x,
                        y,
                        grass_type.to_string()
                    )
                })
            })
            .collect::<Vec<String>>()
            .join("")
    }
}
