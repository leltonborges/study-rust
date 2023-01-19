pub fn conteudo() {
    let context_path = ler_arquivo(String::from("/man/abc"));
    match &context_path {
        Some(value) => println!("{}",value),
        None => println!("Not found")
    };

    println!("debug :: {:?}", &context_path);

    if let Some(value) = context_path{
        println!("right: {}", value);
    }

    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}

pub enum OptionResult<T>{
    Some(T),
    None
}

fn ler_arquivo(path_file: String) -> Option<String> {
    Some(String::from("Conteudo do arquivo"))
}