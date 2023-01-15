const PI: f32 = 3.14;
static GLOBAL_ENV: i8 = 20;

fn soma(age1: u8, age2: u8) -> u8 {
    age1 + age2
}

fn main() {
    let idade: i32 = 31;
    println!("valor = {}, tamanho = {}", idade, std::mem::size_of_val(&idade));

    let decimal = 2.5;
    println!("Decimal = {}, tamanho = {}", decimal, std::mem::size_of_val(&decimal));

    let boleana = false;
    println!("Boleana = {}, tamanho = {}", boleana, std::mem::size_of_val(&boleana));

    let letra = 'C';
    println!("Char = {}, tamanho = {}", letra, std::mem::size_of_val(&letra));

    println!("PI: {}", PI);
    println!("GLOBAL_ENV: {}", GLOBAL_ENV);

    println!("Idade: {}", soma(13, 5));

    println!("Estruturas de condições");
    estrutura_if();
}

fn estrutura_if() {
    let idade: u8 = 17;
    let autorizado: bool = true;

    if idade > 17 {
        println!("Pode entrar na balata");
    } else if idade < 18 && autorizado {
        println!("Pode entrar com a assinatura do responsavel");
    } else {
        println!("Não pode entrar na balada");
    }

    let condicao = if idade > 17 { "maior" } else { "menor" };
    println!("É {} de idade", condicao);
}