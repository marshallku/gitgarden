use crate::utils::{
    coordinate::{generate_coordinate, Rectangle},
    encode::encode_from_path,
};

use super::renderable::Renderable;

pub struct Home {
    coordinate: Rectangle,
    pub dead_zone: Rectangle,
}

impl Home {
    pub fn new(user_name: &str) -> Self {
        let (x, y) = generate_coordinate(user_name, (80.0, 730.0), (25.0, 70.0), None).unwrap();

        let coordinate = Rectangle {
            x1: x,
            y1: y,
            x2: x + 67.0,
            y2: y + 152.0,
        };

        Self {
            coordinate: coordinate.clone(),
            // Add object size to coordinate
            dead_zone: Rectangle {
                x1: coordinate.x1,
                y1: coordinate.y1,
                x2: coordinate.x2,
                y2: coordinate.y2 + 89.0,
            },
        }
    }
}

impl Renderable for Home {
    fn render(&self) -> String {
        let home = encode_from_path("objects/home.png");
        let road = encode_from_path("objects/stone_road.png");

        format!(
            r#"<image width="151" height="155" x="{}" y="{}" xlink:href="data:image/png;base64,{}" />
            <image width="31" height="89" x="{}" y="{}" xlink:href="data:image/png;base64,{}" />"#,
            self.coordinate.x1,
            self.coordinate.y1,
            home,
            self.coordinate.x2,
            self.coordinate.y2,
            road
        )
    }
}
