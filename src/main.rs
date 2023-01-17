const PI: f32 = 3.14;
static GLOBAL_ENV: i8 = 20;

fn soma(age1: u8, age2: u8) -> u8 {
    age1 + age2
}

fn main() {
    // environment();
    // estrutura_if();
    // repedicoes();
    // match_estrutura();
    // ownership();
    // pattern_matching();
    // matching();
    erros();
}

fn environment() {
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
}

fn estrutura_if() {
    println!("Estruturas de condições");
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

fn repedicoes() {
    println!("Estruturas de repetições");

    let multiplicador: u8 = 8;
    let mut count: u8 = 0;
    while count < 10 {
        count += 1;
        println!("{} x {} = {}", multiplicador, count, count * multiplicador);
    }

    println!("Loops");
    count = 0;
    loop {
        count += 1;
        if count == 5 { continue; }
        println!("{} x {} = {}", multiplicador, count, count * multiplicador);
        if count == 10 { break; }
    }

    println!("For!!");
    for i in 1..11 {
        println!("{} x {} = {}", multiplicador, i, i * multiplicador);
    }

    'outer: for x in 5..50 {
        for y in 0..10 {
            println!("x: {}, y: {}", x, y);
            if x == y {
                println!("break x: {}, y: {}", x, y);
                break 'outer;
            }
        }
    }

    for i in std::iter::repeat(5) {
        println!("turns out {} never stops being 5", i);
        break; // would loop forever otherwise
    }
}

fn match_estrutura() {
    println!("Match");
    let linguagem = "";
    let proposito = match linguagem {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "ALL"
    };

    println!("Proposito: {}", proposito);
}

fn ownership() {
    let mut uma_string = String::from("Lelton");
    rouba(&mut uma_string);
    println!("Dono: {}", uma_string);
}

fn rouba(nome: &mut String) {
    nome.push_str("  Borges");
    println!("Emprestado: {}", nome);
}

fn pattern_matching() {
    for i in 0..20 {
        println!("{}: {}", i, match i {
            1 => "Pouco",
            2 | 3 => "Pouquinho",
            4..=10 => "Um bocado",
            _ if i % 2 == 0 => "Uma boa quantidade",
            _ => "Default"
        });
    }

    for x in 0..10 {
        for y in 0..10 {
            println!("{} : {} = {:?}", x, y, match (x, y) {
                (0, 0) => "Ínicio",
                (0, _) => "Coluno zero",
                (_, _) if x == y => "Iguais",
                _ => "None"
            })
        }
    }
}

fn matching() {
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}

fn erros() {
    match get_result() {
        Ok(s) => println!("Success: {}", s),
        Err(er) => println!("Err: {}", er)
    }
}

fn get_result() -> Result<String, u8> {
    Ok(String::from("Tudo deu certo"))
    // Err(4)
}