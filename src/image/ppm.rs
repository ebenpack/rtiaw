use crate::color::Color;
use crate::image::Image;
use itertools::Itertools;

pub struct PPM {
    pub image_width: i32,
    pub image_height: i32,
    pub image_data: Vec<Vec<Color>>,
}

impl Image<String> for PPM {
    fn render(&self) -> Vec<u8> {
        let mut image = String::default();
        image.push_str(&format!(
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        ));
        for row in &self.image_data {
            image.push_str(&format!(
                "{}\n",
                row.iter()
                    .map(|pixel| format!(
                        "{} {} {}",
                        pixel.hex_red(),
                        pixel.hex_green(),
                        pixel.hex_blue(),
                    ))
                    .intersperse("\n".to_string())
                    .join("")
            ));
        }
        image.as_bytes().to_owned()
    }
}
