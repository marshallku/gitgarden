use std::fmt::Write;

use rustint::Color;

use crate::{constants::render::MASK_CLASS, utils::encode::encode_from_path};

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
        let dirt_color = Color::try_from("#e5c77c").unwrap();
        let grass_color = Color::try_from("#a5c543").unwrap();
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
               <defs>
                    <style>
                        .{} {{
                            mix-blend-mode: color;
                            opacity: 0.8;
                        }}
                    </style>
               </defs>
               <defs>{}</defs>
               <defs>{}</defs>
            "##,
            self.width,
            self.height,
            self.width,
            self.height,
            background_color,
            MASK_CLASS,
            self.register_objects(),
            self.register_masks()
        );

        for object in &self.objects {
            svg.push_str(&object.render());
        }

        svg.push_str("</svg>");

        svg
    }

    fn register_masks(&self) -> String {
        format!(
            r##"
            <mask id="{}" maskUnits="objectBoundingBox" maskContentUnits="objectBoundingBox">
                <rect x="0" y="0" width="1" height="1" fill="#000"/>
                <path d="M0.0625 0.3125h0.06375v-0.0625H0.5v-0.061875L0.5625 0.1875v-0.0625h0.25v0.061875L0.875 0.1875v0.0625h0.0625v0.0625h0.0625v0.438125L0.9375 0.75v0.0625h-0.3125v0.0625h-0.0625v0.0625h-0.121875L0.4375 0.875h-0.0625v-0.0625h-0.25v-0.0625h-0.0625v-0.4375z" fill="#fff"/>
            </mask>
            "##,
            Objects::FlowerFour.get_mask_id().unwrap()
        )
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
