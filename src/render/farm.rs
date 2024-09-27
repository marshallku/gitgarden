use std::fmt::Write;

use crate::utils::{color::Rgb, encode::encode_from_path};

use super::{objects::Objects, renderable::Renderable};

pub struct Farm {
    width: u32,
    height: u32,
    progress: f32,
    objects: Vec<Box<dyn Renderable>>,
}

impl Farm {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            progress: 0.0,
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: impl Renderable + 'static) {
        self.objects.push(Box::new(object));
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress;
    }

    pub fn render(&self) -> String {
        let dirt_color = Rgb::from_hex("#e5c77c").unwrap();
        let grass_color = Rgb::from_hex("#a5c543").unwrap();
        let background_color = dirt_color.interpolate(&grass_color, self.progress);

        let mut svg = format!(
            r##"
            <svg
                xmlns="http://www.w3.org/2000/svg"
                xmlns:xlink="http://www.w3.org/1999/xlink"
                viewBox="0 0 {} {}"
                fill="none"
                style="width: {}px; height: {}px;"
            >
               <rect width="100%" height="100%" fill="{}" />
               <defs>{}</defs>
            "##,
            self.width,
            self.height,
            self.width,
            self.height,
            background_color,
            self.register_objects()
        );

        for object in &self.objects {
            svg.push_str(&object.render());
        }

        svg.push_str("</svg>");

        svg
    }

    fn register_objects(&self) -> String {
        Objects::iter().fold(String::new(), |mut acc, object| {
            let path = object.to_path();
            let encoded = encode_from_path(&path);
            let (width, height) = object.to_size();

            write!(
                acc,
                r#"<image id="{}" width="{}" height="{}" xlink:href="data:image/png;base64,{}" />"#,
                object.to_string(),
                width,
                height,
                encoded
            )
            .expect("Writing to string should not fail");

            acc
        })
    }
}
