use crate::utils::coordinate::{must_generate_coordinate, Rectangle};

use super::{objects::Objects, renderable::Renderable};

pub struct Trees {
    user_name: String,
    width: u32,
    count: i32,
    dead_zone: Rectangle,
}

impl Trees {
    pub fn new(user_name: &str, width: u32, count: i32, dead_zone: &Rectangle) -> Self {
        Self {
            user_name: user_name.to_string(),
            width,
            count,
            dead_zone: dead_zone.clone(),
        }
    }
}

impl Renderable for Trees {
    fn render(&self) -> String {
        let mut trees = String::new();
        let mut coords: Vec<(f64, f64)> = (0..self.count)
            .map(|i| {
                must_generate_coordinate(
                    format!("{}-tree-{}", self.user_name, i),
                    (5.0, self.width as f64 - 50.0),
                    (5.0, 230.0),
                    Some(&self.dead_zone),
                )
            })
            .collect();

        coords.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        for (x, y) in coords {
            let (tree_kind, _) = must_generate_coordinate(
                format!("{}-tree-kind-{}", self.user_name, x),
                (1.0, 2.0),
                (1.0, 2.0),
                None,
            );
            let tree = match tree_kind as u32 {
                1 => Objects::TreeOne,
                _ => Objects::TreeTwo,
            };

            trees.push_str(&format!(
                r##"<use x="{}" y="{}" xlink:href="#{}" />"##,
                x,
                y,
                tree.to_string()
            ));
        }

        trees
    }
}
