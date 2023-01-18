use colors::{color, color::Color};
use matrix::matriz;
use week_days::{week, week::WeekDay};

mod week_days;
mod matrix;
mod colors;

fn main() {
    // arrays();
    // matrix();
    println!("Fim de semana: {}", week::fim_de_semana(WeekDay::QUARTA));
    println!("Cor: {}", color::cores(Color::BLUE));
    println!("Cor: {}", color::cores(Color::RgbColor(3, 6, 1)));
    println!("Cor: {}", color::cores(Color::CymkColor { cyan: 10, magenta: 95, yellow: 15, black: 64 }));
}