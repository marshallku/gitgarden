use crate::utils::{color::Rgb, encode::encode_from_path};

use super::{objects::Objects, renderable::Renderable};

pub struct Farm {
    width: u32,
    height: u32,
    objects: Vec<Box<dyn Renderable>>,
}

impl Farm {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: impl Renderable + 'static) {
        self.objects.push(Box::new(object));
    }

    pub fn render(&self) -> String {
        let background_color = Rgb::from_hex("#a5c543").unwrap();
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
            background_color.to_hex(),
            self.register_objects()
        );

        for object in &self.objects {
            svg.push_str(&object.render());
        }

        svg.push_str("</svg>");

        svg
    }

    fn register_objects(&self) -> String {
        Objects::iter()
            .map(|object| {
                let path = object.to_path();
                let encoded = encode_from_path(&path);
                format!(
                    r#"<image id="{}" width="{}" height="{}" xlink:href="data:image/png;base64,{}" />"#,
                    object.to_string(),
                    object.to_size().0,
                    object.to_size().1,
                    encoded
                )
            })
            .collect()
    }
}
