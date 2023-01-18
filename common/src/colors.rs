pub mod color {
    use std::fmt::{Display, format};

    pub fn cores(cor: Color) -> String {
        match cor {
            Color::RED => String::from("Vermelho"),
            Color::GREEN => String::from("Verde"),
            Color::BLUE => String::from("Azul"),
            Color::RgbColor(red, green, blue) => format!("RGB({}, {}, {})", red, green, blue),
            Color::CymkColor { cyan, magenta, yellow, black }
                    => format!("cymk({},{},{},{})", cyan, magenta, yellow, black)
        }
    }

    #[allow(dead_code)]
    pub enum Color {
        RED,
        GREEN,
        BLUE,
        RgbColor(u8, u8, u8),
        CymkColor { cyan: u8, magenta: u8, yellow: u8, black: u8 },
    }
}