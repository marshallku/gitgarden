use crate::utils::{coordinate::generate_coordinate, encode::encode_from_path};

use super::renderable::Renderable;

pub struct Home {
    user_name: String,
}

impl Home {
    pub fn new(user_name: &str) -> Self {
        Self {
            user_name: user_name.to_string(),
        }
    }
}

impl Renderable for Home {
    fn render(&self) -> String {
        let (x, y) =
            generate_coordinate(&self.user_name, (80.0, 730.0), (25.0, 70.0), None).unwrap();
        let home = encode_from_path("objects/home.png");
        let road = encode_from_path("objects/stone_road.png");

        format!(
            r#"<image width="151" height="155" x="{}" y="{}" xlink:href="data:image/png;base64,{}" />
            <image width="31" height="89" x="{}" y="{}" xlink:href="data:image/png;base64,{}" />"#,
            x,
            y,
            home,
            x + 67.0,
            y + 152.0,
            road
        )
    }
}
