
pub mod week {
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
