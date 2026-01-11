use macroquad::prelude::*;
use macroquad::ui::{root_ui, Skin};

pub struct SkinLoader {
    skin: Skin
}

impl SkinLoader {
    pub async fn new() -> SkinLoader {
        let label_style = root_ui()
            .style_builder()
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .font_size(25)
            .build();

        let window_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(include_bytes!("assets/window_background.png"), None).unwrap())
            .build();

        let button_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(include_bytes!("assets/button_background.png"), None).unwrap())
            .background_margin(RectOffset::new(8.0, 8.0, 8.0, 8.0))
            .background_hovered(Image::from_file_with_format(include_bytes!("assets/button_hovered_background.png"), None).unwrap())
            .background_clicked(Image::from_file_with_format(include_bytes!("assets/button_clicked_background.png"), None).unwrap())
            .text_color(Color::from_rgba(180, 180, 100, 255))
            .font_size(25)
            .build();

        let checkbox_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(include_bytes!("assets/checkbox_background.png"), None).unwrap())
            .background_hovered(Image::from_file_with_format(include_bytes!("assets/checkbox_hovered_background.png"), None).unwrap())
            .background_clicked(Image::from_file_with_format(include_bytes!("assets/checkbox_clicked_background.png"), None).unwrap())
            .build();

        let editbox_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(include_bytes!("assets/editbox_background.png"), None).unwrap())
            .background_margin(RectOffset::new(2.0, 2.0, 2.0, 2.0))
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .font_size(25)
            .build();

        let combobox_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(include_bytes!("assets/combobox_background.png"), None).unwrap())
            .background_margin(RectOffset::new(4.0, 25.0, 6.0, 6.0))
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .color(Color::from_rgba(210, 210, 210, 255))
            .font_size(25)
            .build();

        let skin = Skin {
            window_style,
            button_style,
            label_style,
            checkbox_style,
            editbox_style,
            combobox_style,
            ..root_ui().default_skin()
        };

        SkinLoader{skin}
    }

    pub fn get_skin(&self) -> Skin {
        let skin = self.skin.clone();

        skin
    }
}