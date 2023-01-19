use crate::colors::{color, color::Color};
use crate::week_days::week::WeekDay;

pub fn week_and_days() {
    println!("Fim de semana: {}", week::fim_de_semana(WeekDay::QUARTA));
    println!("Cor: {}", color::cores(Color::BLUE));
    println!("Cor: {}", color::cores(Color::RgbColor(3, 6, 1)));
    println!("Cor: {}", color::cores(Color::CymkColor { cyan: 10, magenta: 95, yellow: 15, black: 64 }));
}


mod week {
    pub fn fim_de_semana(dia: WeekDay) -> bool {
        match dia {
            WeekDay::DOMINGO | WeekDay::SABADO => true,
            _ => false
        }
    }

    pub enum WeekDay {
        DOMINGO,
        SEGUNDA,
        TERCA,
        QUARTA,
        QUINTA,
        SEXTA,
        SABADO,
    }
}
